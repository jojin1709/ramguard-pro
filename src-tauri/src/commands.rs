use std::sync::{Arc, Mutex};
use sysinfo::System;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;

use crate::memory::{self, MemoryStatus, OptimizeResult};
use crate::process_manager::{self, ProcessInfo};
use crate::settings::{self, Settings, SettingsState};

pub struct SysState(pub Arc<Mutex<System>>);

#[tauri::command]
pub fn get_memory_status() -> MemoryStatus {
    memory::get_memory_status()
}

#[tauri::command]
pub fn get_process_list(
    sys_state: State<SysState>,
    settings_state: State<SettingsState>,
) -> Vec<ProcessInfo> {
    let whitelist = settings_state.0.lock().unwrap().whitelist.clone();
    let mut sys = sys_state.0.lock().unwrap();
    process_manager::list_processes(&mut sys, &whitelist)
}

#[tauri::command]
pub fn get_admin_status() -> bool {
    memory::is_elevated()
}

/// Helper function to perform memory optimization logic.
pub fn execute_optimize(
    app: &AppHandle,
    sys_mutex: Arc<Mutex<System>>,
    settings_mutex: Arc<Mutex<Settings>>,
) -> OptimizeResult {
    let before = memory::get_memory_status();
    memory::enable_debug_privilege();

    let (clear_standby, whitelist) = {
        let s = settings_mutex.lock().unwrap();
        (s.clear_standby_on_optimize, s.whitelist.clone())
    };

    let pids = {
        let mut sys = sys_mutex.lock().unwrap();
        process_manager::trimmable_pids(&mut sys, &whitelist)
    };

    let mut trimmed = 0u32;
    let mut failed = 0u32;
    for pid in pids {
        if memory::trim_process_working_set(pid) {
            trimmed += 1;
        } else {
            failed += 1;
        }
    }

    let standby_cleared = if clear_standby {
        memory::clear_standby_list()
    } else {
        false
    };

    let after = memory::get_memory_status();
    let freed_raw = (before.used_mb as i64 - after.used_mb as i64).max(0);

    // Update cumulative stats in settings
    {
        let mut s = settings_mutex.lock().unwrap();
        s.total_freed_mb += freed_raw as u64;
        s.total_optimizations += 1;
        if let Ok(dir) = app.path().app_data_dir() {
            let _ = settings::save(&dir, &s);
        }
    }

    OptimizeResult {
        processes_trimmed: trimmed,
        processes_failed: failed,
        standby_list_cleared: standby_cleared,
        freed_mb: freed_raw,
        is_admin: memory::is_elevated(),
    }
}

/// Runs a full optimize pass asynchronously in a blocking thread so the UI window thread never freezes.
#[tauri::command]
pub async fn optimize_now(
    app: AppHandle,
    sys_state: State<'_, SysState>,
    settings_state: State<'_, SettingsState>,
) -> Result<OptimizeResult, String> {
    let sys_mutex = sys_state.0.clone();
    let settings_mutex = settings_state.0.clone();
    let app_handle = app.clone();

    let result = tauri::async_runtime::spawn_blocking(move || {
        execute_optimize(&app_handle, sys_mutex, settings_mutex)
    })
    .await
    .map_err(|e| format!("Optimization task failed: {e}"))?;

    Ok(result)
}

#[tauri::command]
pub fn kill_process(state: State<SysState>, pid: u32) -> Result<(), String> {
    let mut sys = state.0.lock().unwrap();
    process_manager::kill_process(&mut sys, pid)
}

#[tauri::command]
pub fn get_settings(state: State<SettingsState>) -> Settings {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    state: State<SettingsState>,
    new_settings: Settings,
) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    settings::save(&dir, &new_settings).map_err(|e| e.to_string())?;

    let autolaunch = app.autolaunch();
    let currently_enabled = autolaunch.is_enabled().unwrap_or(false);
    if new_settings.launch_on_startup && !currently_enabled {
        let _ = autolaunch.enable();
    } else if !new_settings.launch_on_startup && currently_enabled {
        let _ = autolaunch.disable();
    }

    *state.0.lock().unwrap() = new_settings;
    Ok(())
}

#[tauri::command]
pub fn reset_stats(
    app: AppHandle,
    state: State<SettingsState>,
) -> Result<(), String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let mut s = state.0.lock().unwrap();
    s.total_freed_mb = 0;
    s.total_optimizations = 0;
    settings::save(&dir, &s).map_err(|e| e.to_string())?;
    Ok(())
}

