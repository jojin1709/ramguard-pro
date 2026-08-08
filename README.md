> [!NOTE]
> **[RAMGuard Pro v0.1.0 is officially released](https://github.com/jojin1709/ramguard-pro/releases/tag/v0.1.0)** — High-Performance Real Windows Memory Optimization Engine by **JOJIN JOHN**.

<div align="center">

<img src="docs/icon.png" width="96" height="96" alt="RAMGuard Pro Logo" />

# 🛡️ RAMGuard Pro

### Proprietary Windows Memory Optimizer Engine
**Developed by JOJIN JOHN**

[![Developer: JOJIN JOHN](https://img.shields.io/badge/Developer-JOJIN_JOHN-gold?style=for-the-badge)](https://github.com/jojin1709)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows_10%2F11-0078D6?style=for-the-badge&logo=windows)](https://microsoft.com/windows)
[![Engine: Rust](https://img.shields.io/badge/Engine-Rust_Kernel_API-DE3A1E?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Framework: Tauri v2 + React](https://img.shields.io/badge/Framework-Tauri_v2_%2B_React_18-61DAFB?style=for-the-badge&logo=react)](https://tauri.app/)
[![Website: Live Website](https://img.shields.io/badge/Website-jojin1709.github.io%2Framguard--pro-3fa9a0?style=for-the-badge&logo=googlechrome)](https://jojin1709.github.io/ramguard-pro/)
[![Security Policy](https://img.shields.io/badge/Security-Policy-blue?style=for-the-badge)](SECURITY.md)
[![Contributing](https://img.shields.io/badge/Guidelines-Feedback-orange?style=for-the-badge)](CONTRIBUTING.md)
[![Code of Conduct](https://img.shields.io/badge/Contributor_Covenant-2.1-purple?style=for-the-badge)](CODE_OF_CONDUCT.md)
[![License: Proprietary](https://img.shields.io/badge/License-Proprietary_All_Rights_Reserved-red?style=for-the-badge)](LICENSE)

RAMGuard Pro is a high-performance Windows desktop application engineered to safely reclaim RAM, optimize process working sets, and purge cached system standby memory using kernel-level NT APIs.

---

### 🌐 [**Visit Live Interactive Website (jojin1709.github.io/ramguard-pro)**](https://jojin1709.github.io/ramguard-pro/)

---

### 📦 Download Ready-to-Run Executables (.exe)

| Package Name | File Type | Download Link | Description |
| --- | --- | --- | --- |
| 💿 **RAMGuard Pro Setup** | NSIS Setup Installer | [**Download Setup (.exe)**](releases/RAMGuard-Pro-Setup.exe) | Guided setup installer with desktop shortcut and start menu options |
| ⚡ **RAMGuard Pro Portable** | Standalone Executable | [**Download Portable (.exe)**](releases/RAMGuard-Pro.exe) | Single-file portable executable — no installation required |

---

</div>

> [!TIP]
> **Administrator Elevation:** Run RAMGuard Pro as Administrator to enable full System Standby List purging (`NtSetSystemInformation`). Standard mode will trim process working sets safely (`EmptyWorkingSet`).

---

## Table of Contents

- [What is RAMGuard Pro?](#what-is-ramguard-pro)
- [Download & Setup Guide](#download--setup-guide)
- [Key Capabilities](#key-capabilities)
- [Architecture](#architecture)
- [Safety & System Shield Protections](#safety--system-shield-protections)
- [Proprietary License & Ownership](#proprietary-license--ownership)
- [Developer Credit](#developer-credit)

---

## What is RAMGuard Pro?

**RAMGuard Pro** is an advanced Windows memory optimization utility developed by **JOJIN JOHN**. Unlike placebo memory cleaner tools that corrupt system cache or force artificial allocation spikes, RAMGuard Pro employs standard, verified Windows operating system kernel APIs:

1. **Working-Set Trimming (`EmptyWorkingSet`)**: Programmatically signals the Windows memory manager to release unallocated or idle memory pages held by running background applications.
2. **Standby Memory Purging (`NtSetSystemInformation`)**: Purges cached system standby memory (`MemoryPurgeStandbyList`) that Windows hoards as file cache.
3. **Smart Process Guard**: Features an immutable protection list (`csrss.exe`, `lsass.exe`, `explorer.exe`, `dwm.exe`, `svchost.exe`, etc.) to guarantee zero system crashes or BSODs.

---

## Download & Setup Guide

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

## Safety & System Shield Protections

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

## Developer Credit

<div align="center">

### **Developed by JOJIN JOHN**
*Copyright © 2026 JOJIN JOHN. All Rights Reserved.*

</div>
