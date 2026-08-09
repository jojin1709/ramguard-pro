export interface MemoryStatus {
  total_mb: number;
  available_mb: number;
  used_mb: number;
  percent_used: number;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  memory_mb: number;
  cpu_usage: number;
  is_protected: boolean;
  is_dangerous: boolean;
  is_suggested_cleanup: boolean;
  is_whitelisted: boolean;
}

export interface OptimizeResult {
  processes_trimmed: number;
  processes_failed: number;
  standby_list_cleared: boolean;
  freed_mb: number;
  is_admin: boolean;
}

export interface Settings {
  auto_trigger_percent: number;
  check_interval_secs: number;
  clear_standby_on_optimize: boolean;
  launch_on_startup: boolean;
  minimize_to_tray: boolean;
  whitelist: string[];
  total_freed_mb: number;
  total_optimizations: number;
}

