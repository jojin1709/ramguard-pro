> [!NOTE]
> **[RAMGuard Pro v0.1.0 is officially released](releases/RAMGuard-Pro-Setup.exe)** — High-Performance Real Windows Memory Optimization Engine by **JOJIN JOHN**.

<div align="center">

# 🛡️ RAMGuard Pro

### Proprietary Windows Memory Optimizer Engine
**Developed by JOJIN JOHN**

[![Platform: Windows](https://img.shields.io/badge/Platform-Windows_10%2F11-0078D6?style=for-the-badge&logo=windows)](https://microsoft.com/windows)
[![Engine: Rust](https://img.shields.io/badge/Engine-Rust_Kernel_API-DE3A1E?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![UI: React + Tauri](https://img.shields.io/badge/Framework-Tauri_v2_%2B_React_18-61DAFB?style=for-the-badge&logo=react)](https://tauri.app/)
[![Website: Live Docs](https://img.shields.io/badge/Website-Interactive_Docs-3fa9a0?style=for-the-badge&logo=html5)](docs/index.html)
[![License: Proprietary](https://img.shields.io/badge/License-Proprietary_All_Rights_Reserved-gold?style=for-the-badge)](LICENSE)

RAMGuard Pro is a high-performance Windows desktop application engineered to safely reclaim RAM, optimize process working sets, and purge cached system standby lists using kernel-level NT APIs.

---

### 🌐 [**View Animated Landing Website (`docs/index.html`)**](docs/index.html)

---

### 📦 Download Ready-to-Run Executables (.exe)

| Download | File Type | Description |
| --- | --- | --- |
| 💿 [**Download NSIS Setup Installer (.exe)**](releases/RAMGuard-Pro-Setup.exe) | Setup Installer | Guided installer with desktop shortcut and start menu options |
| ⚡ [**Download Portable Executable (.exe)**](releases/RAMGuard-Pro.exe) | Standalone Executable | Portable single-file executable, no installation required |

---

</div>

> [!TIP]
> **Administrator Elevation:** Run RAMGuard Pro as Administrator to enable full System Standby List purging (`NtSetSystemInformation`). Standard mode will trim process working sets safely (`EmptyWorkingSet`).

---

## Table of Contents

- [What is RAMGuard Pro?](#what-is-ramguard-pro)
- [Download & Installation](#download--installation)
- [Key Capabilities](#key-capabilities)
- [Architecture](#architecture)
- [Safety & Protection Shields](#safety--protection-shields)
- [Proprietary License & Ownership](#proprietary-license--ownership)
- [Developer Credit](#developer-credit)

---

## What is RAMGuard Pro?

**RAMGuard Pro** is an advanced Windows memory optimization utility developed by **JOJIN JOHN**. Unlike placebo memory cleaner apps that corrupt system cache or force artificial paging, RAMGuard Pro employs standard, verified Windows operating system kernel APIs:

1. **Working-Set Trimming (`EmptyWorkingSet`)**: Programmatically signals Windows memory manager to release unallocated or idle memory pages held by running applications.
2. **Standby Memory Purging (`NtSetSystemInformation`)**: Purges cached system standby memory (`MemoryPurgeStandbyList`) that Windows hoards for file caching.
3. **Smart Process Guard**: Features an immutable protection list (`csrss.exe`, `lsass.exe`, `explorer.exe`, `dwm.exe`, `svchost.exe`, etc.) to guarantee zero system crashes or BSODs.

---

## Download & Installation

You do not need to compile or install any development tools to use RAMGuard Pro. Pre-built, optimized `.exe` binaries are provided directly in the repository:

### 1. Using the Setup Installer (`RAMGuard-Pro-Setup.exe`)
1. Download [`releases/RAMGuard-Pro-Setup.exe`](releases/RAMGuard-Pro-Setup.exe).
2. Double-click the installer to launch the setup wizard.
3. Choose whether to install for all users or current user, and toggle **Desktop Shortcut** creation.
4. Launch **RAMGuard Pro** from your desktop or start menu.

### 2. Using the Portable Executable (`RAMGuard-Pro.exe`)
1. Download [`releases/RAMGuard-Pro.exe`](releases/RAMGuard-Pro.exe).
2. Run `RAMGuard-Pro.exe` directly. Windows UAC will request Administrator elevation.

---

## Key Capabilities

- **⚡ Instant Memory Reclamation**: Reduces overall system RAM load within seconds with a single click.
- **🛡️ Real-Time Admin Mode Detection**: Auto-detects token elevation status and displays live indicator badges (**🛡️ Administrator Mode** vs **⚠️ User Mode**).
- **🔍 Interactive Process Manager & Search**: Search processes by name or PID, with instant filtering for **Suggested Cleanup**, **System Protected**, and **Caution / Danger** apps.
- **🤖 Quiet Background Watcher**: Auto-optimizes RAM in the background when memory consumption crosses user-configured thresholds (e.g. 85%).
- **📌 System Tray Quick Access**: Minimizes to system tray. Left-clicking or double-clicking the tray icon restores the UI instantly; right-clicking provides instant trigger & exit options.
- **🖼️ Custom NSIS Setup Installer**: Bundled setup installer with custom splash graphics, desktop shortcuts, and Start menu shortcuts.

---

## Architecture

RAMGuard Pro separates kernel-level native operations from presentation logic using an asynchronous IPC architecture:

```text
       ┌──────────────────────────────────────┐
       │   React 18 / TS Desktop Frontend     │
       │   (Gauge, Process Table, Settings)   │
       └──────────────────┬───────────────────┘
                          │
                          │ Tauri v2 IPC (invoke)
                          ▼
       ┌──────────────────────────────────────┐
       │     Rust Application Engine          │
       │   (commands.rs & main.rs Watcher)    │
       └──────────┬────────────────┬──────────┘
                  │                │
                  ▼                ▼
       ┌──────────────────┐  ┌──────────────────┐
       │ EmptyWorkingSet  │  │ NtSetSystemInfo  │
       │ (PSAPI Trim Pass)│  │ (Standby Purge)  │
       └──────────────────┘  └──────────────────┘
```

---

## Safety & Protection Shields

> [!IMPORTANT]
> The following core OS shell processes are hard-coded as **System Protected** and can NEVER be terminated or trimmed by RAMGuard Pro:
> - `system`, `registry`, `smss.exe`, `csrss.exe`, `wininit.exe`, `winlogon.exe`, `services.exe`, `lsass.exe`, `svchost.exe`, `explorer.exe`, `dwm.exe`, `ctfmon.exe`, `conhost.exe`, `ramguard-pro.exe`.

---

## Proprietary License & Ownership

> [!IMPORTANT]
> **RAMGuard Pro is NOT open-source software.**

Copyright (c) 2026 **JOJIN JOHN**. All Rights Reserved.

This software and all associated source code, binaries, executables, graphics, and documentation are proprietary and confidential property of **JOJIN JOHN**.

- **No Copying or Redistribution**: Copying, modifying, publishing, distributing, sublicensing, selling, decompiling, or reverse engineering of this software or any part thereof is strictly prohibited without explicit written permission from **JOJIN JOHN**.
- **All Rights Reserved**: All title, ownership rights, and intellectual property rights remain exclusively with **JOJIN JOHN**.

---

## Developed By

<div align="center">

### **Developed by JOJIN JOHN**
*Copyright © 2026 JOJIN JOHN. All Rights Reserved.*

</div>
