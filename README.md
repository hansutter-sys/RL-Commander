# RLCommander ⚡

A high-performance, modern dual-pane **Total Commander** clone with native **SFTP** support built with **Tauri v2**, **Rust**, **Svelte 5**, **TypeScript**, and **Tailwind CSS**.

![RLCommander Dual Pane UI](https://raw.githubusercontent.com/tauri-apps/tauri/dev/app-icon.png)

## 🌟 Key Features

- 📂 **Dual-Pane File Architecture**: Classic side-by-side active/inactive panel paradigm with independent paths, sorting, and virtualized rendering.
- 🚀 **Native SFTP Engine**: Integrated high-speed SFTP client powered by Rust (`ssh2` / Tokio async tasks). Connect directly to remote servers inside any pane.
- ⚡ **High-Performance Virtual Scrolling**: Smooth rendering of directories with 100,000+ files using `@tanstack/svelte-virtual`.
- ⌨️ **Pure Total Commander Keyboard Navigation**:
  - `Tab`: Toggle active pane focus (Left ↔ Right)
  - `ArrowUp` / `ArrowDown`: Move row cursor
  - `Space` / `Insert`: Toggle selection on current item
  - `Enter`: Open folder or launch file
  - `Backspace`: Move up to parent directory (`..`)
  - `F3`: View File (Modal viewer)
  - `F4`: Edit File
  - `F5`: Copy (Active → Inactive pane)
  - `F6`: Move / Rename (Active → Inactive pane)
  - `F7`: Create Directory (`mkdir`)
  - `F8` / `Delete`: Delete selected files with confirmation
- 🔄 **Non-Blocking Streaming Transfers**: Background file copy & move engine with Tokio `mpsc` channels and live `transfer-progress` events (MB/s speed, ETA, chunked 64KB streaming).
- 🖥️ **Cross-Platform**: Designed for Linux & Windows desktop environments via Tauri v2.

---

## 🛠️ Architecture

```
RL-Commander/
├── src-tauri/                 # Rust Backend (Tauri v2)
│   ├── src/
│   │   ├── main.rs            # Entry point & Tauri plugin setup
│   │   ├── commands.rs        # Tauri IPC command handlers
│   │   └── fs/                # Unified File System Trait
│   │       ├── traits.rs      # FileSystemProvider trait
│   │       ├── local.rs       # Local OS FileSystem provider
│   │       └── sftp.rs        # Native SFTP client provider
└── src/                       # Svelte 5 + TypeScript Frontend
    ├── lib/
    │   ├── types.ts           # Type definitions
    │   ├── tauri.ts           # Tauri IPC wrapper API
    │   ├── stores/            # Svelte state management
    │   └── components/        # UI components (Pane, FileList, Toolbar, Modals)
    ├── App.svelte             # Main Commander Dual-Pane Shell
    └── main.ts
```

---

## 🚀 Quick Start & Development

### Prerequisites

- **Rust** (1.75+): Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js** (v18+) & **pnpm**: `npm install -g pnpm`
- **Linux Dependencies** (Debian/Ubuntu):
  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/RL-Commander.git
cd RL-Commander

# Install frontend dependencies
pnpm install

# Run in Tauri v2 development mode
pnpm tauri dev
```

### Build for Production

```bash
# Build desktop binary (AppImage, deb, or exe)
pnpm tauri build
```

---

## 📄 License

MIT License © 2026 RLCommander Contributors
