use std::sync::Mutex;
use sysinfo::System;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;

use crate::memory::{self, MemoryStatus, OptimizeResult};
use crate::process_manager::{self, ProcessInfo};
use crate::settings::{self, Settings, SettingsState};

pub struct SysState(pub Mutex<System>);

#[tauri::command]
pub fn get_memory_status() -> MemoryStatus {
    memory::get_memory_status()
}

#[tauri::command]
pub fn get_process_list(state: State<SysState>) -> Vec<ProcessInfo> {
    let mut sys = state.0.lock().unwrap();
    process_manager::list_processes(&mut sys)
}

#[tauri::command]
pub fn get_admin_status() -> bool {
    memory::is_elevated()
}

/// Runs a full optimize pass: enable debug privilege, trim every trimmable
/// process's working set, optionally purge the standby list.
#[tauri::command]
pub fn optimize_now(state: State<SysState>, settings_state: State<SettingsState>) -> OptimizeResult {
    let before = memory::get_memory_status();
    memory::enable_debug_privilege();

    let mut sys = state.0.lock().unwrap();
    let pids = process_manager::trimmable_pids(&mut sys);
    drop(sys);

    let mut trimmed = 0u32;
    let mut failed = 0u32;
    for pid in pids {
        if memory::trim_process_working_set(pid) {
            trimmed += 1;
        } else {
            failed += 1;
        }
    }

    let clear_standby = settings_state.0.lock().unwrap().clear_standby_on_optimize;
    let standby_cleared = if clear_standby {
        memory::clear_standby_list()
    } else {
        false
    };

    let after = memory::get_memory_status();
    let freed_raw = before.used_mb as i64 - after.used_mb as i64;
    OptimizeResult {
        processes_trimmed: trimmed,
        processes_failed: failed,
        standby_list_cleared: standby_cleared,
        freed_mb: freed_raw.max(0),
        is_admin: memory::is_elevated(),
    }
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
