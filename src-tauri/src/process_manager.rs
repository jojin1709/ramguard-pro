//! Process enumeration + safe termination.
//!
//! Anything that touches other processes is guarded by PROTECTED_PROCESSES —
//! those are never killable through this app, full stop, regardless of what
//! the frontend sends. Everything else in "suggested cleanup" is opt-in only;
//! nothing is ever auto-killed without the user clicking it.

use serde::Serialize;
use std::collections::HashSet;
use sysinfo::{Pid, System};

#[derive(Serialize, Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: u64,
    pub cpu_usage: f32,
    pub is_protected: bool,
    pub is_dangerous: bool,
    pub is_suggested_cleanup: bool,
}

/// Core OS / shell processes that must never be killed or trimmed by this tool,
/// no matter what the frontend or a bloat-list match says. Killing any of these
/// can blue-screen or log the user out.
const PROTECTED_PROCESSES: &[&str] = &[
    "system",
    "system idle process",
    "registry",
    "smss.exe",
    "csrss.exe",
    "wininit.exe",
    "winlogon.exe",
    "services.exe",
    "lsass.exe",
    "svchost.exe",
    "explorer.exe",
    "dwm.exe",
    "fontdrvhost.exe",
    "sihost.exe",
    "ctfmon.exe",
    "conhost.exe",
    "ramguard-pro.exe",
];

/// Commonly-safe-to-close background bloat: launchers, telemetry helpers, and
/// "always running" tray apps that most people don't need live 24/7. These are
/// only ever *suggested* in the UI — the user picks what actually gets closed.
const SUGGESTED_CLEANUP: &[&str] = &[
    "onedrive.exe",
    "teams.exe",
    "skype.exe",
    "spotify.exe",
    "discord.exe",
    "steam.exe",
    "epicgameslauncher.exe",
    "adobeupdateservice.exe",
    "creativecloud.exe",
    "yourphone.exe",
    "phoneexperiencehost.exe",
    "widgets.exe",
    "gamebar.exe",
    "gamebarftserver.exe",
    "cortana.exe",
    "msedgewebview2.exe",
];

/// Processes that are NOT system-critical but are risky or important to kill.
/// Ending these can break security, audio, input, networking, or crash apps.
/// The UI will show a strong ⚠ danger warning before allowing the user to end them.
const DANGEROUS_PROCESSES: &[&str] = &[
    // Windows Defender / Security
    "msmpeng.exe",          // Windows Defender Antivirus
    "mssense.exe",          // Windows Defender ATP sensor
    "securityhealthservice.exe",
    "securityhealthsystray.exe",
    "nissrv.exe",           // Defender network inspection
    "mpcmdrun.exe",
    // Audio — killing causes immediate audio loss
    "audiodg.exe",
    "audiosrv.exe",
    // Input / display
    "TabTip.exe",           // Touch keyboard
    "TextInputHost.exe",
    // System utilities being used
    "taskmgr.exe",
    "regedit.exe",
    "mmc.exe",
    // Antivirus / firewall (third-party)
    "avp.exe",              // Kaspersky
    "avgnt.exe",            // Avira
    "avguard.exe",
    "mbam.exe",             // Malwarebytes
    "mbamtray.exe",
    "bdagent.exe",          // Bitdefender
    "ekrn.exe",             // ESET
    "egui.exe",
    "nortonsecurity.exe",
    "ns.exe",
    "mcshield.exe",         // McAfee
    "mfemms.exe",
    // Networking
    "dnscache.exe",
    "iphlpsvc.exe",
    // Driver hosts
    "DriverStore.exe",
    "WUDFHost.exe",
    // Runtime brokers
    "RuntimeBroker.exe",
    "ShellExperienceHost.exe",
];

fn normalize(name: &str) -> String {
    name.to_lowercase()
}

pub fn list_processes(sys: &mut System) -> Vec<ProcessInfo> {
    sys.refresh_all();
    let mut out: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc_)| {
            let name = proc_.name().to_string_lossy().to_string();
            let key = normalize(&name);
            ProcessInfo {
                pid: pid.as_u32(),
                name,
                memory_mb: proc_.memory() / 1024 / 1024,
                cpu_usage: proc_.cpu_usage(),
                is_protected: PROTECTED_PROCESSES.contains(&key.as_str()),
                is_dangerous: DANGEROUS_PROCESSES.contains(&key.as_str()),
                is_suggested_cleanup: SUGGESTED_CLEANUP.contains(&key.as_str()),
            }
        })
        .collect();

    out.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
    out
}

pub fn trimmable_pids(sys: &mut System) -> Vec<u32> {
    sys.refresh_all();
    let protected: HashSet<&str> = PROTECTED_PROCESSES.iter().copied().collect();
    sys.processes()
        .iter()
        .filter_map(|(pid, proc_)| {
            let key = normalize(&proc_.name().to_string_lossy());
            if protected.contains(key.as_str()) {
                None
            } else {
                Some(pid.as_u32())
            }
        })
        .collect()
}

/// Kills a process by PID. Refuses outright if it's in PROTECTED_PROCESSES,
/// even if the caller somehow bypassed the UI safeguard.
pub fn kill_process(sys: &mut System, pid: u32) -> Result<(), String> {
    sys.refresh_all();
    let p = Pid::from_u32(pid);
    let Some(process) = sys.process(p) else {
        return Err("Process not found — it may have already exited.".into());
    };
    let key = normalize(&process.name().to_string_lossy());
    let display_name = process.name().to_string_lossy().to_string();
    if PROTECTED_PROCESSES.contains(&key.as_str()) {
        return Err(format!("Cannot end protected system process: {display_name}"));
    }

    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
            if handle.is_null() {
                return Err(format!(
                    "Access denied — cannot open {display_name}. Try running RAMGuard Pro as administrator."
                ));
            }
            let ok = TerminateProcess(handle, 1);
            CloseHandle(handle);
            if ok == 0 {
                return Err(format!("Failed to terminate {display_name}."));
            }
        }
        Ok(())
    }

    #[cfg(not(windows))]
    {
        if process.kill() {
            Ok(())
        } else {
            Err(format!("Failed to kill {display_name}."))
        }
    }
}

