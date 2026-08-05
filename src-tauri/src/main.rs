// Prevents the console window from appearing — applies to both debug and release.
#![windows_subsystem = "windows"]


mod commands;
mod memory;
mod process_manager;
mod settings;

use std::sync::Mutex;
use std::time::Duration;
use sysinfo::System;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager, WindowEvent};

use commands::SysState;
use settings::SettingsState;

fn main() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
        use windows_sys::Win32::System::Threading::CreateMutexW;
        use std::ptr::null_mut;

        unsafe {
            let name: Vec<u16> = "Global\\RAMGuardProSingleInstanceMutex\0".encode_utf16().collect();
            let handle = CreateMutexW(null_mut(), 1, name.as_ptr());
            if handle == 0 as _ || GetLastError() == ERROR_ALREADY_EXISTS {
                // Another instance is already running in tray
                std::process::exit(0);
            }
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(SysState(Mutex::new(System::new_all())))
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let loaded = settings::load(&app_data_dir);
            app.manage(SettingsState(Mutex::new(loaded)));

            // --- System tray ---
            let show_item = MenuItem::with_id(app, "show", "Show RAMGuard Pro", true, None::<&str>)?;
            let optimize_item = MenuItem::with_id(app, "optimize", "Optimize Now", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &optimize_item, &quit_item])?;

            TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .tooltip("RAMGuard Pro")
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "optimize" => {
                        let sys_state = app.state::<SysState>();
                        let settings_state = app.state::<SettingsState>();
                        let result = commands::optimize_now(sys_state, settings_state);
                        let _ = app.emit("optimize-complete", result);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // --- Background watcher: auto-optimize when RAM crosses the threshold ---
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                let interval = {
                    let state = handle.state::<SettingsState>();
                    let s = state.0.lock().unwrap();
                    (s.check_interval_secs.max(2), s.auto_trigger_percent)
                };
                std::thread::sleep(Duration::from_secs(interval.0));

                let trigger_percent = interval.1;
                if trigger_percent == 0 {
                    continue; // auto mode disabled
                }
                let status = memory::get_memory_status();
                if status.percent_used >= trigger_percent {
                    let sys_state = handle.state::<SysState>();
                    let settings_state = handle.state::<SettingsState>();
                    let result = commands::optimize_now(sys_state, settings_state);
                    let _ = handle.emit("optimize-complete", result);
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<SettingsState>();
                let minimize = state.0.lock().unwrap().minimize_to_tray;
                if minimize {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_memory_status,
            commands::get_process_list,
            commands::get_admin_status,
            commands::optimize_now,
            commands::kill_process,
            commands::get_settings,
            commands::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running RAMGuard Pro");
}
