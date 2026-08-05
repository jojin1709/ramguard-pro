use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    /// Auto-optimize once RAM usage crosses this percent (0 disables auto mode).
    pub auto_trigger_percent: u32,
    /// How often (seconds) the background watcher checks memory while the app is running.
    pub check_interval_secs: u64,
    /// Also purge the system standby list on every optimize pass (needs admin).
    pub clear_standby_on_optimize: bool,
    pub launch_on_startup: bool,
    pub minimize_to_tray: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_trigger_percent: 85,
            check_interval_secs: 10,
            clear_standby_on_optimize: true,
            launch_on_startup: false,
            minimize_to_tray: true,
        }
    }
}

pub struct SettingsState(pub Mutex<Settings>);

fn settings_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("settings.json")
}

pub fn load(app_data_dir: &PathBuf) -> Settings {
    let path = settings_path(app_data_dir);
    match fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

pub fn save(app_data_dir: &PathBuf, settings: &Settings) -> std::io::Result<()> {
    fs::create_dir_all(app_data_dir)?;
    let path = settings_path(app_data_dir);
    let raw = serde_json::to_string_pretty(settings).unwrap_or_default();
    fs::write(path, raw)
}
