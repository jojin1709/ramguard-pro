interface GaugeProps {
  percent: number;
  usedMb: number;
  totalMb: number;
}

// Arc runs from -125deg to +125deg (250deg sweep), matching the app icon's dial.
const START_ANGLE = -125;
const SWEEP = 250;

function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
  const rad = ((angleDeg - 90) * Math.PI) / 180;
  return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) };
}

function arcPath(cx: number, cy: number, r: number, startAngle: number, endAngle: number) {
  const start = polarToCartesian(cx, cy, r, endAngle);
  const end = polarToCartesian(cx, cy, r, startAngle);
  const largeArc = endAngle - startAngle <= 180 ? 0 : 1;
  return `M ${start.x} ${start.y} A ${r} ${r} 0 ${largeArc} 0 ${end.x} ${end.y}`;
}

function formatGb(mb: number) {
  return (mb / 1024).toFixed(1);
}

export default function Gauge({ percent, usedMb, totalMb }: GaugeProps) {
  const clamped = Math.max(0, Math.min(100, percent));
  const endAngle = START_ANGLE + (SWEEP * clamped) / 100;
  const size = 260;
  const cx = size / 2;
  const cy = size / 2;
  const r = 100;

  const zone = clamped >= 85 ? "hot" : clamped >= 65 ? "warm" : "cool";

  return (
    <div className="gauge">
      <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`}>
        <path
          d={arcPath(cx, cy, r, START_ANGLE, START_ANGLE + SWEEP)}
          className="gauge-track"
          fill="none"
          strokeWidth={14}
          strokeLinecap="round"
        />
        <path
          d={arcPath(cx, cy, r, START_ANGLE, endAngle)}
          className={`gauge-value gauge-value--${zone}`}
          fill="none"
          strokeWidth={14}
          strokeLinecap="round"
        />
        <text x={cx} y={cy - 6} textAnchor="middle" className="gauge-percent">
          {Math.round(clamped)}%
        </text>
        <text x={cx} y={cy + 22} textAnchor="middle" className="gauge-sub">
          {formatGb(usedMb)} / {formatGb(totalMb)} GB
        </text>
      </svg>
    </div>
  );
}
