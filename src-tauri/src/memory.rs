//! Core memory-optimization engine.
//!
//! Two techniques, both are what real tools like RAMMap / EmptyStandbyList use:
//!   1. Per-process working-set trim  -> EmptyWorkingSet (psapi)
//!   2. System standby-list purge     -> NtSetSystemInformation (ntdll, undocumented
//!      but stable and widely used; same call EmptyStandbyList.exe uses)
//!
//! Both need elevated privileges for processes you don't own, which is why the
//! app requests `requireAdministrator` in its manifest (see build.rs).

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct MemoryStatus {
    pub total_mb: u64,
    pub available_mb: u64,
    pub used_mb: u64,
    pub percent_used: u32,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct OptimizeResult {
    pub processes_trimmed: u32,
    pub processes_failed: u32,
    pub standby_list_cleared: bool,
    pub freed_mb: i64,
    pub is_admin: bool,
}

#[cfg(windows)]
mod win {
    use super::*;
    use std::ffi::c_void;
    use std::mem::{size_of, zeroed};
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LUID};
    use windows_sys::Win32::Security::{
        AdjustTokenPrivileges, GetTokenInformation, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES,
        SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_ELEVATION, TokenElevation,
        TOKEN_PRIVILEGES, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    use windows_sys::Win32::System::ProcessStatus::EmptyWorkingSet;
    use windows_sys::Win32::System::Threading::{
        GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION,
        PROCESS_SET_QUOTA,
    };

    fn is_null(h: HANDLE) -> bool {
        h == 0 as HANDLE
    }

    pub fn get_memory_status() -> MemoryStatus {
        unsafe {
            let mut mem: MEMORYSTATUSEX = zeroed();
            mem.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
            GlobalMemoryStatusEx(&mut mem);
            let total_mb = mem.ullTotalPhys / 1024 / 1024;
            let available_mb = mem.ullAvailPhys / 1024 / 1024;
            MemoryStatus {
                total_mb,
                available_mb,
                used_mb: total_mb.saturating_sub(available_mb),
                percent_used: mem.dwMemoryLoad,
            }
        }
    }

    /// Enables a named privilege (e.g. "SeDebugPrivilege") on this process's token.
    /// Required to touch other processes' working sets, and to purge the standby list.
    fn enable_privilege(name: &str) -> bool {
        unsafe {
            let mut token: HANDLE = 0 as HANDLE;
            if OpenProcessToken(
                GetCurrentProcess(),
                TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                &mut token,
            ) == 0
            {
                return false;
            }

            let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let mut luid: LUID = zeroed();
            if LookupPrivilegeValueW(null_mut(), wide.as_ptr(), &mut luid) == 0 {
                CloseHandle(token);
                return false;
            }

            let mut tp = TOKEN_PRIVILEGES {
                PrivilegeCount: 1,
                Privileges: [LUID_AND_ATTRIBUTES {
                    Luid: luid,
                    Attributes: SE_PRIVILEGE_ENABLED,
                }],
            };

            let ok = AdjustTokenPrivileges(token, 0, &mut tp, 0, null_mut(), null_mut());
            CloseHandle(token);
            ok != 0
        }
    }

    pub fn enable_debug_privilege() -> bool {
        enable_privilege("SeDebugPrivilege")
    }

    /// Trims one process's working set (forces Windows to page out what it isn't
    /// actively using). This is the same thing Task Manager's "Reduce working set"
    /// tricks do, applied programmatically.
    pub fn trim_process_working_set(pid: u32) -> bool {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_SET_QUOTA, 0, pid);
            if is_null(handle) {
                return false;
            }
            let ok = EmptyWorkingSet(handle);
            CloseHandle(handle);
            ok != 0
        }
    }

    // ---- Undocumented NT API for purging the system standby list ----
    // This is the exact mechanism Sysinternals-style "free RAM" tools use.
    // SystemInformationClass 80 = SystemMemoryListInformation
    // Command 4 = MemoryPurgeStandbyList
    #[link(name = "ntdll")]
    extern "system" {
        fn NtSetSystemInformation(
            system_information_class: i32,
            system_information: *mut c_void,
            system_information_length: u32,
        ) -> i32;
    }

    const SYSTEM_MEMORY_LIST_INFORMATION: i32 = 80;
    const MEMORY_FLUSH_MODIFIED_LIST: u32 = 3;
    const MEMORY_PURGE_STANDBY_LIST: u32 = 4;

    pub fn clear_standby_list() -> bool {
        // Needs SeProfileSingleProcessPrivilege and SeIncreaseQuotaPrivilege in addition to admin rights.
        enable_privilege("SeProfileSingleProcessPrivilege");
        enable_privilege("SeIncreaseQuotaPrivilege");
        unsafe {
            // First, flush dirty modified pages to the standby list
            let mut flush_command = MEMORY_FLUSH_MODIFIED_LIST;
            let _ = NtSetSystemInformation(
                SYSTEM_MEMORY_LIST_INFORMATION,
                &mut flush_command as *mut _ as *mut c_void,
                size_of::<u32>() as u32,
            );

            // Then, purge the standby list
            let mut purge_command = MEMORY_PURGE_STANDBY_LIST;
            let status = NtSetSystemInformation(
                SYSTEM_MEMORY_LIST_INFORMATION,
                &mut purge_command as *mut _ as *mut c_void,
                size_of::<u32>() as u32,
            );
            // NTSTATUS 0 = STATUS_SUCCESS
            status == 0
        }
    }

    pub fn is_elevated() -> bool {
        unsafe {
            let mut token: HANDLE = 0 as HANDLE;
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
                return false;
            }
            let mut elevation: TOKEN_ELEVATION = zeroed();
            let mut size = 0u32;
            let ok = GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut c_void,
                size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            );
            CloseHandle(token);
            ok != 0 && elevation.TokenIsElevated != 0
        }
    }
}

#[cfg(not(windows))]
mod win {
    use super::*;

    pub fn get_memory_status() -> MemoryStatus {
        MemoryStatus {
            total_mb: 0,
            available_mb: 0,
            used_mb: 0,
            percent_used: 0,
        }
    }
    pub fn enable_debug_privilege() -> bool {
        false
    }
    pub fn is_elevated() -> bool {
        false
    }
    pub fn trim_process_working_set(_pid: u32) -> bool {
        false
    }
    pub fn clear_standby_list() -> bool {
        false
    }
}

pub use win::{clear_standby_list, enable_debug_privilege, get_memory_status, is_elevated, trim_process_working_set};
