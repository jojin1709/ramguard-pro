import { Settings } from "../types";

interface Props {
  settings: Settings;
  onChange: (s: Settings) => void;
  onSave: () => void;
}

export default function SettingsPanel({ settings, onChange, onSave }: Props) {
  return (
    <div className="panel">
      <div className="panel-header">
        <h2>Auto-optimize</h2>
        <span className="panel-hint">Runs quietly in the background while the app is open</span>
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

      <button className="btn-primary" onClick={onSave}>
        Save settings
      </button>
    </div>
  );
}
