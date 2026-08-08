export default function LogoIcon({ size = 32 }: { size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 32 32"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className="brand-logo-svg"
    >
      <rect
        width="32"
        height="32"
        rx="8"
        fill="#121520"
        stroke="#e3c25f"
        strokeWidth="1.2"
        strokeOpacity="0.6"
      />
      {/* Background Gauge Track */}
      <path
        d="M 7.5 21 A 9.5 9.5 0 1 1 24.5 21"
        fill="none"
        stroke="#232838"
        strokeWidth="3.2"
        strokeLinecap="round"
      />
      {/* Active Gold Gauge Arc */}
      <path
        d="M 7.5 21 A 9.5 9.5 0 0 1 22.5 11.5"
        fill="none"
        stroke="#e3c25f"
        strokeWidth="3.2"
        strokeLinecap="round"
      />
      {/* Needle Line */}
      <line
        x1="16"
        y1="16"
        x2="21.5"
        y2="10.5"
        stroke="#FFFFFF"
        strokeWidth="2"
        strokeLinecap="round"
      />
      {/* Center Pivot Pin */}
      <circle cx="16" cy="16" r="2.5" fill="#e3c25f" />
    </svg>
  );
}
