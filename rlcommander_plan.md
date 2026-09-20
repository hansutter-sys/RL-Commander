# 🚀 Genomförandeplan: RLCommander v1.1

> [!NOTE]
> **RLCommander** är en modern, högpresterande dual-pane filhanterare (Total Commander-klon) byggd med **Tauri v2**, **Rust**, **Svelte 5**, **TypeScript** och **Tailwind CSS**, komplett med inbyggd **SFTP-klient**, filredigerare, sökfunktion och tangentbordsstöd (F3–F8 / Alt+F7).

---

## 📋 Översikt över Etapper & Agentfördelning

```mermaid
graph TD
    A[Huvudagent / Orkestrator] --> B[Fas 1: Rust Backend & SFTP Engine]
    A --> C[Fas 2: Svelte 5 Frontend & Virtualized Dual-Pane]
    A --> D[Fas 3: Keybinding Controller & Modaler F3-F8]
    A --> E[Fas 5: Redigerare, Sök & Tvåvägs-SFTP v1.1]
    B --> F[Fas 4: Integrering & Verifiering]
    C --> F
    D --> F
    E --> F
```

---

## 📌 Detaljerad Etappindelning

### 🔧 Fas 1: Rust Backend Engine (`src-tauri/`)
- [x] **Tauri v2 konfiguration**: `Cargo.toml`, `tauri.conf.json`, `build.rs`, `main.rs`, `lib.rs`
- [x] **Unified FileSystem Trait System** (`src-tauri/src/fs/`):
  - `traits.rs`: `FileSystemProvider` trait (`list_directory`, `create_dir`, `remove`, `exists`, `copy_file`, `write_file`, `search_files`, `read_preview`)
  - `local.rs`: Lokal filhantering för Linux (`/`) och Windows enheter (`C:\`) med rekursiv sökning & filskrivning
  - `sftp.rs`: Native SFTP-anslutning via `ssh2` med tvåvägs-filhantering & sökning
- [x] **Tauri IPC Commands & Progress Stream**:
  - `commands.rs`: `list_directory`, `list_drives`, `create_directory`, `delete_items`, `read_file_text`, `write_file_text`, `search_files`, `copy_items_async`
  - Asynkron filöverföringsmotor med Tokio `mpsc` channels och live `transfer-progress` events (MB/s, bytes, status)

---

### 🎨 Fas 2: Svelte 5 Dual-Pane Frontend (`src/`)
- [x] **Konfigurationsfiler & CSS**:
  - `package.json`, `vite.config.ts`, `tsconfig.json`, `tailwind.config.js`, `app.css` (Glassmorphic dark design system)
- [x] **State Management & Typer**:
  - `src/lib/types.ts`: Datastrukturer för `FileItem`, `PaneState`, `TransferProgress`, `SFTPConfig`, `Bookmark`
  - `src/lib/tauri.ts`: IPC-anrop till Rust backend
  - `src/lib/stores/commander.ts`: Tillståndshantering för vänster/höger panel, markeringar, bokmärken och modaler
- [x] **Paneler & Filtabell**:
  - `Pane.svelte`: Header med sökstig, enhetsväljare, live snabbfilter och bokmärken
  - `FileList.svelte`: Virtualiserad filvy med alla filattribut, ikoner och markeringsstöd
  - `Splitter.svelte`: Dragbar avdelare mellan paneler

---

### ⌨️ Fas 3: Tangentbordsstyrare & Modaler (`F3`–`F8` & `Alt+F7`)
- [x] **Tangentbordsnavigering**:
  - `Tab`: Växla aktiv panel (Vänster ↔ Höger)
  - `ArrowUp` / `ArrowDown`: Flytta markör
  - `Space` / `Insert`: Markera/avmarkera fil
  - `Enter`: Öppna mapp/fil
  - `Backspace`: Navigera till föräldramapp (`..`)
  - `Alt+F7`: Sök filer i undermappar
- [x] **Funktionsknappars Modaler & Toolbar**:
  - `Toolbar.svelte`: Nedre verktygsrad (F3–F8)
  - `FileViewerModal.svelte` (`F3` Filvisare)
  - `FileEditorModal.svelte` (`F4` Filredigerare med spara-stöd & `Ctrl+S`)
  - `SearchModal.svelte` (`Alt+F7` Rekursiv fil- & mappsökning)
  - `CreateDirModal.svelte` (`F7` Skapa mapp)
  - `DeleteConfirmModal.svelte` (`F8` Ta bort bekräftelse)
  - `SftpModal.svelte`: SFTP Snabbanslutning (Värd, Port, Användare, Auth)

---

### 🏁 Fas 4: Integrering, Verifiering & Paushantering
- [x] Samla ihop alla moduler och verifiera med kodanalys och strukturkontroll.
- [x] Skapa ett rent repository-state i Git så att du när som helst kan natt-pausa och återuppta utan förlust!
