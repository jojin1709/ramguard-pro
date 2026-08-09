import { useState } from "react";
import { ProcessInfo } from "../types";

interface Props {
  processes: ProcessInfo[];
  onKill: (pid: number, name: string) => void;
  onToggleWhitelist?: (name: string) => void;
}

type FilterCategory = "all" | "suggested" | "system" | "danger" | "whitelisted";

export default function ProcessList({ processes, onKill, onToggleWhitelist }: Props) {
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState<FilterCategory>("all");
  const [showAll, setShowAll] = useState(false);

  const filtered = processes.filter((p) => {
    const matchesSearch =
      p.name.toLowerCase().includes(search.toLowerCase()) ||
      p.pid.toString().includes(search);

    if (!matchesSearch) return false;

    if (category === "suggested") return p.is_suggested_cleanup;
    if (category === "system") return p.is_protected;
    if (category === "danger") return p.is_dangerous;
    if (category === "whitelisted") return p.is_whitelisted;
    return true;
  });

  const displayed = showAll ? filtered : filtered.slice(0, 15);

  return (
    <div className="panel">
      <div className="panel-header">
        <div>
          <h2>Memory Consumers</h2>
          <span className="panel-hint">
            Suggested cleanup items are highlighted — nothing is closed without your permission
          </span>
        </div>
        <div className="panel-actions">
          <input
            type="text"
            className="search-input"
            placeholder="🔍 Search process or PID..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
      </div>

      <div className="filter-bar">
        <button
          className={category === "all" ? "filter-btn filter-btn--active" : "filter-btn"}
          onClick={() => setCategory("all")}
        >
          All ({processes.length})
        </button>
        <button
          className={category === "suggested" ? "filter-btn filter-btn--active" : "filter-btn"}
          onClick={() => setCategory("suggested")}
        >
          Suggested ({processes.filter((p) => p.is_suggested_cleanup).length})
        </button>
        <button
          className={category === "whitelisted" ? "filter-btn filter-btn--active" : "filter-btn"}
          onClick={() => setCategory("whitelisted")}
        >
          🛡️ Whitelisted ({processes.filter((p) => p.is_whitelisted).length})
        </button>
        <button
          className={category === "system" ? "filter-btn filter-btn--active" : "filter-btn"}
          onClick={() => setCategory("system")}
        >
          System ({processes.filter((p) => p.is_protected).length})
        </button>
        <button
          className={category === "danger" ? "filter-btn filter-btn--active" : "filter-btn"}
          onClick={() => setCategory("danger")}
        >
          ⚠ Caution ({processes.filter((p) => p.is_dangerous).length})
        </button>
      </div>

      <div className="process-table">
        <div className="process-row process-row--head">
          <span>Process</span>
          <span>RAM</span>
          <span>CPU</span>
          <span>Action</span>
        </div>
        {displayed.length === 0 ? (
          <div className="process-empty">No matching processes found</div>
        ) : (
          displayed.map((p) => (
            <div className={`process-row ${p.is_dangerous ? "process-row--danger" : ""} ${p.is_whitelisted ? "process-row--whitelisted" : ""}`} key={p.pid}>
              <span className="process-name">
                <span className="pid-badge">PID {p.pid}</span>
                <span className="proc-title">{p.name}</span>
                {p.is_whitelisted && <span className="badge badge--whitelisted">🛡️ whitelisted</span>}
                {p.is_suggested_cleanup && <span className="badge">suggested</span>}
                {p.is_protected && <span className="badge badge--protected">system</span>}
                {p.is_dangerous && <span className="badge badge--danger">⚠ danger</span>}
              </span>
              <span className="mono">{p.memory_mb.toLocaleString()} MB</span>
              <span className="mono">{p.cpu_usage.toFixed(1)}%</span>
              <span className="action-buttons">
                {!p.is_protected && (
                  <>
                    {onToggleWhitelist && (
                      <button
                        className={`btn-ghost ${p.is_whitelisted ? "btn-ghost--whitelisted" : ""}`}
                        onClick={() => onToggleWhitelist(p.name)}
                        title={p.is_whitelisted ? "Remove from RAM optimization whitelist" : "Whitelist app (prevent lag when switching back)"}
                      >
                        {p.is_whitelisted ? "🛡️ Whitelisted" : "+ Whitelist"}
                      </button>
                    )}
                    <button
                      className={p.is_dangerous ? "btn-ghost btn-ghost--danger" : "btn-ghost"}
                      onClick={() => onKill(p.pid, p.name)}
                    >
                      End Process
                    </button>
                  </>
                )}
              </span>
            </div>
          ))
        )}
      </div>

      {filtered.length > 15 && (
        <div className="panel-footer">
          <button className="btn-secondary" onClick={() => setShowAll(!showAll)}>
            {showAll ? "Show Top 15" : `Show All (${filtered.length})`}
          </button>
        </div>
      )}
    </div>
  );
}


