# CLAUDE.md — Villainy Project Context

This file provides persistent context for Claude Code sessions on the Villainy project.
Read this before any audit, feature work, or debugging session.

Full planning documents are in `docs/`:
- `docs/villainy-scaffold.md` — full repo structure, setup steps, implementation order
- `docs/unified-launcher-blueprint.md` — full project blueprint and pillar breakdown

---

## What Villainy Is

A unified Linux game launcher that consolidates Heroic (Epic, GOG, Amazon) and Lutris
into a single interface. The user manages their entire game library across all stores
from one place. Proton-GE handles Windows game compatibility.

Opposite to Heroic in name and intent — Heroic wraps CLI tools with an Electron GUI,
Villainy does the same with a Tauri + React frontend and a Rust backend.

**License:** GPLv3

---

## Stack

| Layer | Technology |
|---|---|
| UI Framework | Tauri v2 |
| Frontend | React + TypeScript |
| Backend / Logic | Rust |
| Compatibility | Proton-GE (community build) |
| CLI Tools | Legendary, GOGdl, Nile, Lutris CLI |

---

## The Five Core Pillars

### 1. CLI Integration
One Rust module per CLI tool in `src-tauri/src/cli/`:
- `legendary.rs` — Epic Games (auth, install, list, launch)
- `gogdl.rs` — GOG Games
- `nile.rs` — Amazon Games
- `lutris.rs` — Lutris CLI + pga.db access

All modules expose clean async functions. Commands in `src-tauri/src/commands/` are
thin bridges only — real logic lives in `cli/`. Nothing else in the app touches
CLI tools directly.

### 2. Session Management
- Auth tokens are owned by each CLI tool — Villainy never stores raw tokens
- A local JSON file (600 permissions) tracks auth state per store
- Rust validates all sessions on startup before React renders
- Re-auth flows are non-destructive — one store expiring doesn't affect others
- Optional encryption of the session file via OS keyring

### 3. UI Rendering (Tauri + React)
- React is purely presentational — it calls Rust via `invoke()`, receives typed data, renders
- All CLI calls, filesystem operations, and process spawning originate in Rust
- IPC via Tauri Commands (React → Rust) and Tauri Events (Rust → React)
- Single listener builder `useEvent.ts` owns all event lifecycle — no raw `listen()` calls
  scattered across components. Always call unlisten on unmount.

### 4. Download Management
- Downloads are delegated to each CLI tool — no custom HTTP download code
- `download/queue.rs` deduplicates requests before they reach the CLI layer
- Progress streamed from CLI stdout → parsed → emitted as Tauri events → React UI
- Disk space validated before install begins

### 5. Game Launching + Compatibility
- Runner selection: native Linux → run directly, Windows game → Proton-GE
- Proton-GE auto-installs if not found (fetched from GE-Proton GitHub releases API)
- One Wine prefix per game stored at `~/.local/share/villainy/prefixes/{game_id}/`
- Launch command assembled in `launch/builder.rs` from Game struct + resolved settings
- Pre-launch checks: game path, Proton version, prefix validity, session active, not already running
- Crash detection: non-zero exit code → capture stderr tail → write per-game log file

---

## Architecture Principles

- **Commands are thin.** `commands/` files only bridge invoke() to the correct module.
- **React is presentational only.** No filesystem, no process spawning, no session files.
- **One listener builder.** `useEvent.ts` handles all listen/unlisten lifecycle.
- **Session owned by Rust.** Frontend only asks for status, never reads the file.
- **Deduplication in Rust.** queue.rs drops duplicate install requests before CLI.
- **Proton auto-installs.** Never hard-fail on missing runtime — install then proceed.

---

## Repo Structure (abbreviated)

```
villainy/
├── src/
│   ├── components/{Library,GameDetail,DownloadManager,Settings,StoreConnections,shared}
│   ├── hooks/{useLibrary,useDownload,useLaunch,useSessions,useSettings,useEvent}.ts
│   ├── types/index.ts          ← shared type contract, snake_case to match Rust
│   ├── constants/events.ts     ← all Tauri event name strings as constants
│   ├── App.tsx
│   └── main.tsx
└── src-tauri/src/
    ├── main.rs                 ← command registration, app setup
    ├── commands/{library,install,launch,session,settings}.rs
    ├── cli/{legendary,gogdl,nile,lutris,mod}.rs
    ├── session/{store,validator,encryption}.rs
    ├── launch/{runner,proton,prefix,env,builder}.rs
    ├── download/{queue,progress}.rs
    ├── models/mod.rs
    └── utils/{paths,process}.rs
```

---

## Type Contract

TypeScript types in `src/types/index.ts` must mirror Rust structs exactly.
Field names are snake_case. Tauri does no case conversion.

```typescript
export type Store = 'epic' | 'gog' | 'amazon' | 'lutris' | 'sideload';

export interface Game {
    id: string;
    title: string;
    store: Store;
    installed: boolean;
    install_path?: string;
    cover_url?: string;
    playtime_secs: number;
    last_played?: string;
    is_running: boolean;
}

export interface DownloadProgress {
    game_id: string;
    percent: number;
    speed_mbps: number;
    eta_seconds: number;
}

export interface SessionStatus {
    store: Store;
    authenticated: boolean;
    username?: string;
    last_validated: string;
}

export interface Settings {
    default_install_path: string;
    default_proton_version: string;
    max_concurrent_downloads: number;
    enable_gamemode: boolean;
    enable_mangohud: boolean;
}
```

---

## Event Names

All defined as constants in `src/constants/events.ts`. Never use raw strings.

```
download_progress / download_complete / download_failed
install_progress
game_launched / game_exited / game_crashed
session_expired
library_updated
```

---

## Command Surface

```
// Library
get_library, get_library_by_store, get_game_details, refresh_library, search_library

// Install
install_game, uninstall_game, cancel_install, pause_install, resume_install,
verify_game, get_download_queue, get_disk_space

// Launch
launch_game, stop_game, get_running_games, get_launch_log

// Session
get_session_status, authenticate, logout, validate_sessions

// Settings
get_settings, save_settings, get_game_settings, save_game_settings,
get_proton_versions, download_proton_version, get_install_paths
```

---

## Implementation Order (first milestones)

1. `utils/paths.rs` — binary resolution for all CLI tools
2. `cli/legendary.rs` — list_installed() returning parsed JSON
3. `commands/library.rs` — get_library command
4. `hooks/useLibrary.ts` — invoke wrapper
5. `components/Library/` — render game list ← **first visual milestone**
6. `session/store.rs` — session JSON read/write
7. `commands/session.rs` — get_session_status, authenticate
8. `hooks/useEvent.ts` — listener builder
9. `download/queue.rs` — deduplication guard
10. `download/progress.rs` — stdout → Tauri event pipeline
11. `launch/proton.rs` — Proton-GE detection + auto-install
12. `launch/builder.rs` — launch command constructor
13. `commands/launch.rs` — launch_game end to end ← **first playable milestone**

---

## Known Open Questions

- Legendary as Python library vs subprocess — direct import is cleaner but adds
  Python runtime dependency. Subprocess is simpler for now.
- Custom GOGdl reimplementation — deferred to post-MVP. Use GOGdl CLI first.
- GOGdl/Nile session auth flows — less documented, needs hands-on testing.
- Proton-GE injection for non-Legendary games (GOGdl/Nile) — approach TBD.
