# ⚡ wm7tools — Now Written in Zig ⚡

<div align="center">

```
 ██╗    ██╗███╗   ███╗███████╗████████╗ ██████╗  ██████╗ ██╗     ███████╗
 ██║    ██║████╗ ████║╚════██║╚══██╔══╝██╔═══██╗██╔═══██╗██║     ██╔════╝
 ██║ █╗ ██║██╔████╔██║    ██╔╝   ██║   ██║   ██║██║   ██║██║     ███████╗
 ██║███╗██║██║╚██╔╝██║   ██╔╝    ██║   ██║   ██║██║   ██║██║     ╚════██║
 ╚███╔███╔╝██║ ╚═╝ ██║   ██║     ██║   ╚██████╔╝╚██████╔╝███████╗███████║
  ╚══╝╚══╝ ╚═╝     ╚═╝   ╚═╝     ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝╚══════╝
```

> **Windows Mobile 7 Forensic & ROM Extraction Suite**
> *Ground-up rewrite in 100% pure Zig 🦎 — zero C, zero Rust, zero compromise.*

[![Zig](https://img.shields.io/badge/zig-0.16.0-F7A41D?style=for-the-badge&logo=zig&logoColor=white)](https://ziglang.org)
[![Zero Deps](https://img.shields.io/badge/dependencies-zero-brightgreen?style=for-the-badge)](#)
[![Speed](https://img.shields.io/badge/optimize-ReleaseFast-blue?style=for-the-badge)](#)
[![Memory](https://img.shields.io/badge/memory-manually_managed-red?style=for-the-badge)](#)

</div>

---

## 🦎 Why Zig?

Because **Zig doesn't hide from you.**

No hidden allocations. No runtime. No garbage collector deciding your fate at 3am. No borrow checker playing politics with your pointers. Just you, your bytes, and a compiler that tells you exactly what's happening.

Zig is:
- 🧠 **Explicit** — you see every allocation, every branch, every error path
- ⚡ **Fast** — compiled to native code with `ReleaseFast`, no runtime overhead
- 🔬 **Transparent** — `comptime` magic instead of macro dark arts
- 🛠️ **Surgical** — perfect for binary forensics where every byte matters

---

## 🔬 What Is This?

**wm7tools** is a forensic suite for analysing and extracting data from **Windows Mobile 7 ROM images** (`.nb`, `.esco`, raw NAND dumps). These tools let you peer deep into the binary guts of WM7 devices — partition tables, NK kernel images, embedded file systems, and more.

---

## ⚔️ The Arsenal

### 📊 `wmpartinfo` — Partition Table Decoder

> *X-ray vision for ROM images.*

Parses the binary WMSTORE partition table embedded in a ROM image and dumps every field in human-readable form — magic bytes, GUIDs, wchar partition names, sector offsets, sizes, partition types, and unknown fields.

```
WMSTORE:
  Name: DISK
  GUID: {xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}
  Max Partition Count: 0x6
  ...

WMPART 0:
  Name: MANOS
  Offset: 0x6 (@ 0xc00)
  Size: 0x200 (0x40000)
  Partition Type: BOOT (0x20)
  ...
```

**Usage:**
```bash
# Windows
bin\wmpartinfo.exe path\to\image.nb

# Unix / macOS
bin/wmpartinfo path/to/image.nb
```

---

### 📦 `wmnkextract` — NK Image Extractor

> *Reaches inside the kernel and pulls out everything it finds.*

Locates the NK (kernel) partition inside a ROM image — whether it's a raw NK binary or embedded in a WMSTORE structure — resolves virtual base addresses, iterates the TOC (Table of Contents), and extracts all embedded modules and files to an `XIP/` directory on disk.

```
Base: 0x80000000
DLLs: 0x80001000-0x80200000
Phys: 0x80000000-0x80500000
RAM:  0xa0000000-0xa4000000 (Free @ 0xa0010000)
Modules: 142, Files: 63, Copyentries: 12

0x80001000 = Module: nk.exe (size: 0x4a200, attr: 0x20)
0x80050000 = Module: coredll.dll (size: 0x9f000, attr: 0x20)
...
0x800f0000 = File: autorun.exe (size: 0x1800, comp: 0x1800, attr: 0x20)
```

**Usage:**
```bash
# Windows
bin\wmnkextract.exe path\to\image.nb

# Unix / macOS
bin/wmnkextract path/to/image.nb
```

Extracted files land in `./XIP/` in the current working directory.

---

## 🚀 Building

No package manager. No `build.json`. No dependency hell. Just Zig.

**Requires:** [Zig 0.16.0](https://ziglang.org/download/)

### 🪟 Windows
```cmd
.\build.bat
```

### 🐧 Unix / macOS
```bash
chmod +x build.sh
./build.sh
```

Both scripts compile with `-Doptimize=ReleaseFast` and drop the binaries into `bin/`.

Or build manually:
```bash
zig build -Doptimize=ReleaseFast
```

---

## 🧩 Source Layout

```
wm7tools/
├── src/
│   ├── guid.zig          # GUID parsing & fmt
│   ├── mbr.zig           # MBR header structures
│   ├── mmap.zig          # File loading + WcharString formatter
│   ├── wmnk.zig          # NK ROM header, TOC, file entry structures
│   ├── wmpartitions.zig  # WMSTORE / WMPART binary decoders
│   ├── wmpartinfo.zig    # Binary: partition table reporter
│   └── wmnkextract.zig   # Binary: NK image extractor
├── build.zig             # Zig build script
├── build.bat             # Windows build helper
└── build.sh              # Unix build helper
```

---

## 🔐 Design Philosophy

| Concern | Approach |
|---|---|
| **Memory** | Explicit allocation, every `alloc` has a matching `free` |
| **Errors** | Zig error unions — every failure path is visible and handled |
| **I/O** | Zig 0.16.0 `std.Io` interface — injected, swappable, testable |
| **Dependencies** | Zero. Pure `std`. No third-party crates, packages, or modules |
| **Safety** | Bounds checks in debug, optimised away in `ReleaseFast` |
| **Portability** | Compiles for Windows, Linux, macOS from one codebase |

---

## 📜 Supported Structures

| Structure | Description |
|---|---|
| `MBR` | Master Boot Record (512 bytes) with 4 partition entries |
| `WMSTORE` | WM7 store header — magic, GUID, name, partition count |
| `WMPART` | WM7 partition entry — name, offset, size, type, timestamps |
| `NK ROM Header` | CE ROM header with physical/virtual base, module/file counts |
| `TOC Entry` | Module table-of-contents entry (32 bytes) |
| `Files Entry` | Embedded file entry (28 bytes) with compression metadata |

---

## 🤝 Contributing

Spotted a missing structure? Got a ROM image that breaks parsing? PRs and issues are welcome.

---

<div align="center">

*Built with 💛 and a healthy respect for raw bytes.*
*No runtime. No GC. No mercy.*

**🦎 Zig. Just Zig.**

</div>
