# ⚡️ wm7tools-rs ⚡️

> **Next-Generation, Blazing-Fast, Ultra-Safe Memory Forensic & ROM Extraction Suite** 🚀  
> *Re-engineered from the ground up in 100% pure Rust 🦀 with zero dependencies!*

---

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Safety](https://img.shields.io/badge/memory-100%25_safe-brightgreen.svg)](#)
[![Speed](https://img.shields.io/badge/performance-turbocharged-blue.svg)](#)

Welcome to **wm7tools-rs** — the ultimate, turbocharged rejuvenation of the classic Windows Mobile 7 forensic suite. We've completely vaporized all legacy C code, banished potential segfaults to the shadow realm, and birthed a hyper-optimized parsing matrix that runs with the absolute safety of modern Rust.

---

## 🌟 Supercharged Features

*   🦀 **100% Rustified Core**: Fully memory-safe parser with zero unsafe code and zero external crate dependencies.
*   ⚡️ **Blazing Fast**: Hand-crafted, zero-copy byte-slice deserialization engine engineered for maximum throughput.
*   🛡️ **Bulletproof Security**: Bounds-checked to the teeth. Active protection against buffer overflows and malicious ROM payloads.
*   💻 **Universal Portability**: Cross-platform automation scripts for Windows (`build.bat`) and Linux/macOS (`build.sh`).
*   🧩 **Modular Architecture**: Reusable library design allowing easy integration into larger forensic utilities.

---

## 🛠️ The Power Grid (Binary Arsenal)

### 📊 `wmpartinfo`
> *X-ray vision for your ROM images.*  
Decodes and prints the internal structure of the WM partition table (WMSTORE), formats complex GUIDs with absolute precision, and translates wchar partition identifiers.

```bash
$ ./bin/wmpartinfo <path/to/image>
```

### 📦 `wmnkextract`
> *Hyper-extraction engine for NK partitions.*  
Scans MBR headers and partitions, identifies key XIP RAM regions, guesses virtual mapping base addresses, iterates through DLL modules/TOC entries, and automatically reconstructs and extracts internal files.

```bash
$ ./bin/wmnkextract <path/to/image>
```

---

## 🚀 Ignition Sequence (Quick Start)

Build the entire ecosystem in one click with our automated build scripts!

### 🪟 Windows (CMD / PowerShell)
```cmd
> .\build.bat
```

### 🐧 Unix / macOS (Bash / Zsh)
```bash
$ chmod +x build.sh
$ ./build.sh
```

All compiled binaries will be cleanly deposited into the local `bin/` directory.

---

## 🧪 Deep Diagnostics (Unit Tests)

Verify the integrity of the parsing matrix anytime:
```bash
$ cargo test
```

---

## 🤝 Join the Revolution

Got ideas to make this even more warp-speed? Pull requests are always welcome! Let's build the safest ROM tools together. 🌐

*Crafted with 💖 and agentic intelligence by the DeepMind Antigravity Team.*
