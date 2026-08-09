import { useState } from "react";
import { Settings } from "../types";

interface Props {
  settings: Settings;
  onChange: (s: Settings) => void;
  onSave: () => void;
  onResetStats?: () => void;
}

const POPULAR_PRESETS = [
  { label: "VS Code", exe: "code.exe" },
  { label: "Visual Studio", exe: "devenv.exe" },
  { label: "IntelliJ IDEA", exe: "idea64.exe" },
  { label: "PyCharm", exe: "pycharm64.exe" },
  { label: "Chrome", exe: "chrome.exe" },
  { label: "Steam", exe: "steam.exe" },
  { label: "Discord", exe: "discord.exe" },
];

export default function SettingsPanel({ settings, onChange, onSave, onResetStats }: Props) {
  const [newExe, setNewExe] = useState("");

  const handleAddWhitelist = (exeName: string) => {
    const trimmed = exeName.trim().toLowerCase();
    if (!trimmed) return;
    const exe = trimmed.endsWith(".exe") ? trimmed : `${trimmed}.exe`;
    if (!settings.whitelist.some((item) => item.toLowerCase() === exe)) {
      onChange({
        ...settings,
        whitelist: [...settings.whitelist, exe],
      });
    }
    setNewExe("");
  };

  const handleRemoveWhitelist = (exeName: string) => {
    onChange({
      ...settings,
      whitelist: settings.whitelist.filter((item) => item.toLowerCase() !== exeName.toLowerCase()),
    });
  };

  const formattedFreed =
    settings.total_freed_mb >= 1024
      ? `${(settings.total_freed_mb / 1024).toFixed(2)} GB`
      : `${settings.total_freed_mb} MB`;

  return (
    <div className="settings-container" style={{ display: "flex", flexDirection: "column", gap: "20px" }}>
      {/* Auto-optimize Panel */}
      <div className="panel">
        <div className="panel-header">
          <div>
            <h2>Auto-optimize</h2>
            <span className="panel-hint">Runs quietly in the background while the app is open</span>
          </div>
        </div>

        <div className="settings-grid">
          <label className="settings-row">
            <span>Trigger when RAM usage reaches</span>
            <div className="settings-control">
              <input
                type="range"
                min={0}
                max={95}
                step={5}
                value={settings.auto_trigger_percent}
                onChange={(e) =>
                  onChange({ ...settings, auto_trigger_percent: Number(e.target.value) })
                }
              />
              <span className="mono">
                {settings.auto_trigger_percent === 0 ? "off" : `${settings.auto_trigger_percent}%`}
              </span>
            </div>
          </label>

          <label className="settings-row">
            <span>Check every</span>
            <div className="settings-control">
              <input
                type="number"
                min={2}
                value={settings.check_interval_secs}
                onChange={(e) =>
                  onChange({ ...settings, check_interval_secs: Number(e.target.value) })
                }
              />
              <span>seconds</span>
            </div>
          </label>

          <label className="settings-row settings-row--toggle">
            <span>Clear standby memory list on optimize</span>
            <input
              type="checkbox"
              checked={settings.clear_standby_on_optimize}
              onChange={(e) =>
                onChange({ ...settings, clear_standby_on_optimize: e.target.checked })
              }
            />
          </label>

          <label className="settings-row settings-row--toggle">
            <span>Minimize to tray on close</span>
            <input
              type="checkbox"
              checked={settings.minimize_to_tray}
              onChange={(e) => onChange({ ...settings, minimize_to_tray: e.target.checked })}
            />
          </label>
        </div>
      </div>

      {/* App Whitelist Panel */}
      <div className="panel">
        <div className="panel-header">
          <div>
            <h2>App Whitelist 🛡️</h2>
            <span className="panel-hint">
              Whitelisted processes will never be memory-trimmed (prevents lag spikes when switching back to IDEs or games)
            </span>
          </div>
        </div>

        <div className="whitelist-section">
          <div className="whitelist-input-row">
            <input
              type="text"
              className="search-input whitelist-input"
              placeholder="e.g. code.exe, visualstudio.exe..."
              value={newExe}
              onChange={(e) => setNewExe(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") handleAddWhitelist(newExe);
              }}
            />
            <button className="btn-secondary" onClick={() => handleAddWhitelist(newExe)}>
              + Add App
            </button>
          </div>

          <div className="preset-chips">
            <span className="preset-label">Quick Add Presets:</span>
            {POPULAR_PRESETS.map((preset) => {
              const isAdded = settings.whitelist.some(
                (item) => item.toLowerCase() === preset.exe.toLowerCase()
              );
              return (
                <button
                  key={preset.exe}
                  className={`chip ${isAdded ? "chip--added" : ""}`}
                  onClick={() => handleAddWhitelist(preset.exe)}
                  disabled={isAdded}
                >
                  {preset.label} {isAdded ? "✓" : "+"}
                </button>
              );
            })}
          </div>

          <div className="whitelist-tags">
            {settings.whitelist.length === 0 ? (
              <span className="process-empty">No processes whitelisted yet</span>
            ) : (
              settings.whitelist.map((exe) => (
                <span className="whitelist-tag" key={exe}>
                  🛡️ {exe}
                  <button
                    className="tag-remove"
                    onClick={() => handleRemoveWhitelist(exe)}
                    title="Remove from whitelist"
                  >
                    ✕
                  </button>
                </span>
              ))
            )}
          </div>
        </div>
      </div>

      {/* Statistics & Actions Panel */}
      <div className="panel">
        <div className="panel-header">
          <div>
            <h2>Optimization Statistics</h2>
            <span className="panel-hint">Cumulative memory savings since app installation</span>
          </div>
        </div>

        <div className="stats-summary-row">
          <div className="stat-summary-item">
            <span className="stat-label">Total RAM Cleared:</span>
            <span className="mono stat-highlight">{formattedFreed}</span>
          </div>
          <div className="stat-summary-item">
            <span className="stat-label">Optimizations Performed:</span>
            <span className="mono">{settings.total_optimizations}</span>
          </div>
          {onResetStats && (
            <button className="btn-ghost btn-ghost--danger" onClick={onResetStats}>
              Reset Counter
            </button>
          )}
        </div>
      </div>

      <div style={{ display: "flex", justifyContent: "flex-end" }}>
        <button className="btn-primary" onClick={onSave} style={{ width: "200px" }}>
          Save Settings
        </button>
      </div>
    </div>
  );
}

