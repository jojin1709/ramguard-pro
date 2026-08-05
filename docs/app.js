// RAMGuard Pro Animated Landing Page JavaScript

document.addEventListener("DOMContentLoaded", () => {
  const gaugeCircle = document.getElementById("gaugeCircle");
  const gaugePercentText = document.getElementById("gaugePercentText");
  const gaugeSubText = document.getElementById("gaugeSubText");
  const valInUse = document.getElementById("valInUse");
  const valAvail = document.getElementById("valAvail");
  const btnDemoOptimize = document.getElementById("btnDemoOptimize");
  const demoResult = document.getElementById("demoResult");
  const btnCopyCode = document.getElementById("btnCopyCode");

  // Gauge calculation: r=100 -> circumference = 2 * PI * 100 = 628.3
  const CIRCUMFERENCE = 628.3;
  const SWEEP_FRACTION = 250 / 360; // 250 degrees sweep arc
  const MAX_OFFSET = CIRCUMFERENCE * (1 - SWEEP_FRACTION);

  function setGaugePercent(percent) {
    const clamped = Math.max(0, Math.min(100, percent));
    const fraction = (clamped / 100) * SWEEP_FRACTION;
    const offset = CIRCUMFERENCE - CIRCUMFERENCE * fraction;
    gaugeCircle.style.strokeDashoffset = offset;

    // Color zone
    if (clamped >= 80) {
      gaugeCircle.style.stroke = "var(--hot)";
    } else if (clamped >= 60) {
      gaugeCircle.style.stroke = "var(--warm)";
    } else {
      gaugeCircle.style.stroke = "var(--cool)";
    }

    gaugePercentText.textContent = `${Math.round(clamped)}%`;
  }

  // Set initial 78% gauge state
  setGaugePercent(78);

  // Interactive Demo Optimize Button
  let isOptimized = false;
  btnDemoOptimize.addEventListener("click", () => {
    btnDemoOptimize.disabled = true;
    btnDemoOptimize.textContent = "⚡ Optimizing Working Sets…";

    setTimeout(() => {
      if (!isOptimized) {
        setGaugePercent(34);
        gaugeSubText.textContent = "5.4 / 16.0 GB";
        valInUse.textContent = "5.4 GB";
        valAvail.textContent = "10.6 GB";
        demoResult.textContent = "✓ Trimmed 18 processes · Standby list purged · Freed ~7,100 MB!";
        demoResult.classList.remove("hidden");
        btnDemoOptimize.textContent = "🔄 Reset Demo";
        isOptimized = true;
      } else {
        setGaugePercent(78);
        gaugeSubText.textContent = "12.5 / 16.0 GB";
        valInUse.textContent = "12.5 GB";
        valAvail.textContent = "3.5 GB";
        demoResult.classList.add("hidden");
        btnDemoOptimize.textContent = "⚡ Optimize Now (Demo)";
        isOptimized = false;
      }
      btnDemoOptimize.disabled = false;
    }, 900);
  });

  // Copy Rust code snippet
  if (btnCopyCode) {
    btnCopyCode.addEventListener("click", () => {
      const codeSnippet = `pub fn clear_standby_list() -> bool {
    enable_privilege("SeProfileSingleProcessPrivilege");
    enable_privilege("SeIncreaseQuotaPrivilege");
    unsafe {
        let mut command = 4u32;
        let status = NtSetSystemInformation(80, &mut command as *mut _ as _, 4);
        status == 0
    }
}`;
      navigator.clipboard.writeText(codeSnippet).then(() => {
        btnCopyCode.textContent = "✓ Copied!";
        setTimeout(() => {
          btnCopyCode.textContent = "Copy API snippet";
        }, 2000);
      });
    });
  }
});
