import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Gauge from "./components/Gauge";
import ProcessList from "./components/ProcessList";
import SettingsPanel from "./components/SettingsPanel";
import { MemoryStatus, OptimizeResult, ProcessInfo, Settings } from "./types";

type Tab = "dashboard" | "processes" | "settings";

export default function App() {
  const [tab, setTab] = useState<Tab>("dashboard");
  const [mem, setMem] = useState<MemoryStatus | null>(null);
  const [processes, setProcesses] = useState<ProcessInfo[]>([]);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [lastResult, setLastResult] = useState<OptimizeResult | null>(null);
  const [optimizing, setOptimizing] = useState(false);
  const [isAdmin, setIsAdmin] = useState<boolean>(false);

  const [toast, setToast] = useState<{ msg: string; ok: boolean } | null>(null);

  const showToast = (msg: string, ok: boolean) => {
    setToast({ msg, ok });
    setTimeout(() => setToast(null), 3500);
  };

  const refreshMemory = useCallback(async () => {
    const status = await invoke<MemoryStatus>("get_memory_status");
    setMem(status);
  }, []);

  const refreshProcesses = useCallback(async () => {
    const list = await invoke<ProcessInfo[]>("get_process_list");
    setProcesses(list);
  }, []);

  useEffect(() => {
    refreshMemory();
    refreshProcesses();
    invoke<Settings>("get_settings").then(setSettings);
    invoke<boolean>("get_admin_status").then(setIsAdmin).catch(() => setIsAdmin(false));

    const memTimer = setInterval(refreshMemory, 2000);
    const procTimer = setInterval(refreshProcesses, 5000);

    const unlisten = listen<OptimizeResult>("optimize-complete", (event) => {
      setLastResult(event.payload);
      refreshMemory();
      refreshProcesses();
    });

    return () => {
      clearInterval(memTimer);
      clearInterval(procTimer);
      unlisten.then((f) => f());
    };
  }, [refreshMemory, refreshProcesses]);

  const handleOptimize = async () => {
    setOptimizing(true);
    try {
      const result = await invoke<OptimizeResult>("optimize_now");
      setLastResult(result);
      if (result.is_admin !== undefined) {
        setIsAdmin(result.is_admin);
      }
      await Promise.all([refreshMemory(), refreshProcesses()]);
      const standbyMsg = result.standby_list_cleared ? " · Standby list purged" : "";
      showToast(`Trimmed ${result.processes_trimmed} processes${standbyMsg} · freed ~${result.freed_mb} MB`, true);
    } catch (e) {
      showToast(`Optimize failed: ${e}`, false);
    } finally {
      setOptimizing(false);
    }
  };

  const handleKill = async (pid: number, name: string) => {
    const proc = processes.find(p => p.pid === pid);
    const isDangerous = proc?.is_dangerous ?? false;

    const msg = isDangerous
      ? `⚠️ WARNING: "${name}" is a security, audio, or system process!\n\nEnding it may:\n• Disable antivirus / firewall protection\n• Cut audio or break system features\n• Cause instability\n\nAre you SURE you want to end it?`
      : `End process "${name}" (PID ${pid})?\n\nUnsaved work in this app will be lost.`;

    if (!window.confirm(msg)) return;
    try {
      await invoke("kill_process", { pid });
      await refreshProcesses();
      showToast(`✓ Ended ${name}`, true);
    } catch (e) {
      showToast(`✗ ${e}`, false);
    }
  };

  const handleSaveSettings = async () => {
    if (!settings) return;
    try {
      await invoke("save_settings", { newSettings: settings });
      showToast("Settings saved successfully", true);
    } catch (e) {
      showToast(`Failed to save settings: ${e}`, false);
    }
  };

  return (
    <div className="app">
      {toast && (
        <div className={`toast ${toast.ok ? "toast--ok" : "toast--err"}`}>
          {toast.msg}
        </div>
      )}
      <aside className="sidebar">
        <div className="brand">
          <img src="/icon.png" alt="RAMGuard Pro" className="brand-mark-img" />
          <div>
            <div className="brand-title">RAMGuard</div>
            <div className="brand-sub">Pro</div>
          </div>
        </div>

        <div className={`admin-status ${isAdmin ? "admin-status--active" : "admin-status--user"}`}>
          {isAdmin ? "🛡️ Administrator Mode" : "⚠️ User Mode"}
        </div>

        <nav>
          <button className={tab === "dashboard" ? "nav-item nav-item--active" : "nav-item"} onClick={() => setTab("dashboard")}>
            Dashboard
          </button>
          <button className={tab === "processes" ? "nav-item nav-item--active" : "nav-item"} onClick={() => setTab("processes")}>
            Processes
          </button>
          <button className={tab === "settings" ? "nav-item nav-item--active" : "nav-item"} onClick={() => setTab("settings")}>
            Settings
          </button>
        </nav>

        <button className="btn-primary btn-optimize" onClick={handleOptimize} disabled={optimizing}>
          {optimizing ? "Optimizing…" : "Optimize now"}
        </button>

        <div className="sidebar-credit">Developed by JOJIN JOHN</div>
      </aside>

      <main className="content">
        {tab === "dashboard" && mem && (
          <>
            <div className="dashboard-top">
              <Gauge percent={mem.percent_used} usedMb={mem.used_mb} totalMb={mem.total_mb} />
              <div className="stat-cards">
                <div className="stat-card">
                  <span className="stat-label">In use</span>
                  <span className="stat-value">{(mem.used_mb / 1024).toFixed(1)} GB</span>
                </div>
                <div className="stat-card">
                  <span className="stat-label">Available</span>
                  <span className="stat-value">{(mem.available_mb / 1024).toFixed(1)} GB</span>
                </div>
                <div className="stat-card">
                  <span className="stat-label">Total</span>
                  <span className="stat-value">{(mem.total_mb / 1024).toFixed(1)} GB</span>
                </div>
              </div>
            </div>

            {lastResult && (
              <div className="result-strip">
                ✓ Trimmed {lastResult.processes_trimmed} processes
                {lastResult.standby_list_cleared ? " · Standby list purged" : ""}
                {lastResult.freed_mb > 0 ? ` · Freed ~${lastResult.freed_mb} MB` : ""}
              </div>
            )}

            <ProcessList processes={processes} onKill={(pid, name) => handleKill(pid, name)} />
          </>
        )}

        {tab === "processes" && <ProcessList processes={processes} onKill={(pid, name) => handleKill(pid, name)} />}

        {tab === "settings" && settings && (
          <SettingsPanel settings={settings} onChange={setSettings} onSave={handleSaveSettings} />
        )}
      </main>
    </div>
  );
}
