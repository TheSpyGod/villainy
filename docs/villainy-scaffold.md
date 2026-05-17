# Villainy — Project Scaffold

**Repo:** `villainy`  
**Stack:** Tauri v2 + React + TypeScript (frontend) · Rust (backend)  
**License:** GPLv3

---

## 1. Repo Structure

```
villainy/
│
├── src/                                  # React frontend
│   ├── components/
│   │   ├── Library/                      # Unified game grid/list view
│   │   ├── GameDetail/                   # Single game view, launch, settings
│   │   ├── DownloadManager/              # Queue, progress bars, active downloads
│   │   ├── Settings/                     # Global and per-game settings
│   │   ├── StoreConnections/             # Auth status per store, connect/disconnect
│   │   └── shared/                       # Reusable UI primitives (buttons, tiles, etc.)
│   │
│   ├── hooks/
│   │   ├── useLibrary.ts                 # Wraps invoke('get_library') and variants
│   │   ├── useDownload.ts                # Wraps download commands + progress events
│   │   ├── useLaunch.ts                  # Wraps launch commands + game exit events
│   │   ├── useSessions.ts                # Wraps session commands + expiry events
│   │   ├── useSettings.ts                # Wraps settings commands
│   │   └── useEvent.ts                   # Centralised listener builder
│   │
│   ├── types/
│   │   └── index.ts                      # Shared type contract — mirrors Rust structs exactly
│   │
│   ├── constants/
│   │   └── events.ts                     # All Tauri event name strings as typed constants
│   │
│   ├── App.tsx                           # Root layout — sidebar + main content area
│   └── main.tsx                          # Tauri entry point
│
├── src-tauri/
│   └── src/
│       ├── main.rs                       # App setup, command registration
│       │
│       ├── commands/                     # Thin bridge layer — invoke calls land here
│       │   ├── library.rs                # get_library, refresh_library, search_library
│       │   ├── install.rs                # install_game, uninstall, cancel, pause, resume
│       │   ├── launch.rs                 # launch_game, stop_game, get_running, get_log
│       │   ├── session.rs                # get_status, authenticate, logout, validate
│       │   └── settings.rs              # get/save global settings, per-game overrides
│       │
│       ├── cli/                          # All real logic lives here — one module per tool
│       │   ├── legendary.rs              # Epic Games via Legendary CLI
│       │   ├── gogdl.rs                  # GOG via GOGdl CLI
│       │   ├── nile.rs                   # Amazon via Nile CLI
│       │   ├── lutris.rs                 # Lutris CLI + pga.db access
│       │   └── mod.rs                    # Shared CLI helpers (spawn, parse, error types)
│       │
│       ├── session/
│       │   ├── store.rs                  # Read/write local session JSON file
│       │   ├── validator.rs              # Startup validation per store
│       │   └── encryption.rs            # Optional session file encryption
│       │
│       ├── launch/
│       │   ├── runner.rs                 # Runner selection logic (native vs Proton)
│       │   ├── proton.rs                 # Proton-GE detection, download, auto-install
│       │   ├── prefix.rs                 # Wine prefix creation and management
│       │   ├── env.rs                    # Environment variable assembly
│       │   └── builder.rs               # Launch command constructor
│       │
│       ├── download/
│       │   ├── queue.rs                  # Download queue — deduplication, ordering
│       │   └── progress.rs              # Stdout parsing → Tauri event emission
│       │
│       ├── models/
│       │   └── mod.rs                    # Shared Rust structs: Game, Settings, SessionStatus
│       │
│       └── utils/
│           ├── paths.rs                  # Binary resolution, config dir, prefix paths
│           └── process.rs               # Process spawning helpers, cleanup, PID tracking
│
├── Cargo.toml
├── package.json
├── tauri.conf.json
├── tsconfig.json
└── LICENSE                               # GPLv3
```

---

## 2. Architecture Principles

**Commands are thin.** The files in `commands/` do almost nothing except receive the
`invoke()` call and delegate to the relevant `cli/` or domain module. No logic lives
in commands — they are purely the bridge between React and Rust.

**React is purely presentational.** The frontend calls Rust commands via `invoke()`,
receives typed data, renders it. It never touches the filesystem, spawns processes,
or manages session tokens directly.

**One listener builder to rule them all.** Rather than raw `listen()` calls scattered
across components, every event subscription goes through `useEvent.ts`. This hook
internally handles both the `listen()` registration and the `unlisten()` cleanup on
unmount. No component is responsible for its own listener lifecycle.

```typescript
// hooks/useEvent.ts
import { useEffect } from 'react';
import { listen, EventCallback } from '@tauri-apps/api/event';

export function useEvent<T>(
    event: string,
    handler: EventCallback<T>,
    deps: React.DependencyList = []
) {
    useEffect(() => {
        let unlisten: (() => void) | undefined;

        listen<T>(event, handler).then(fn => {
            unlisten = fn;
        });

        // Automatic deallocation on unmount
        return () => { unlisten?.(); };
    }, deps);
}
```

**Session is owned by Rust.** The frontend never reads or writes the session file.
It only asks Rust for session status and triggers auth flows. Rust validates all
tokens on startup before React renders the main UI.

**Download deduplication lives in Rust.** Before any install command reaches the
CLI layer, `queue.rs` checks whether that `game_id` is already queued or active.
Duplicate requests are dropped silently — no error, no second process spawned.

**Proton-GE is auto-installed.** If the pre-launch check in `proton.rs` finds no
Proton-GE version available, it fetches the latest from the GE-Proton GitHub
releases API, installs it, then proceeds with the launch. The user never sees a
hard failure for a missing runtime.

---

## 3. The Type Contract

Define once in `src/types/index.ts`. Field names must be snake_case to match
Rust struct fields exactly — Tauri serialises by field name with no conversion.

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
    last_played?: string;         // ISO timestamp
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
    last_validated: string;       // ISO timestamp
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

## 4. Event Name Constants

```typescript
// constants/events.ts
export const EVENTS = {
    DOWNLOAD_PROGRESS:  'download_progress',
    DOWNLOAD_COMPLETE:  'download_complete',
    DOWNLOAD_FAILED:    'download_failed',
    INSTALL_PROGRESS:   'install_progress',
    GAME_LAUNCHED:      'game_launched',
    GAME_EXITED:        'game_exited',
    GAME_CRASHED:       'game_crashed',
    SESSION_EXPIRED:    'session_expired',
    LIBRARY_UPDATED:    'library_updated',
} as const;
```

---

## 5. Full Command Surface

Every `invoke()` call the frontend will ever make.

```
// Library
get_library()
get_library_by_store({ store })
get_game_details({ game_id, store })
refresh_library({ store })
search_library({ query })

// Install
install_game({ game_id, store, install_path })
uninstall_game({ game_id, store })
cancel_install({ game_id })
pause_install({ game_id })
resume_install({ game_id })
verify_game({ game_id, store })
get_download_queue()
get_disk_space({ path })

// Launch
launch_game({ game_id, store })
stop_game({ game_id })
get_running_games()
get_launch_log({ game_id })

// Session
get_session_status()
authenticate({ store })
logout({ store })
validate_sessions()

// Settings
get_settings()
save_settings({ settings })
get_game_settings({ game_id })
save_game_settings({ game_id, settings })
get_proton_versions()
download_proton_version({ version })
get_install_paths()
```

---

## 6. Setup Steps

### Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# Tauri CLI
cargo install tauri-cli

# System dependencies (Debian/Ubuntu)
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev \
                 libayatana-appindicator3-dev librsvg2-dev patchelf
```

---

### Step 1 — Initialise the Tauri project

```bash
cd villainy
cargo tauri init
```

When prompted:
- App name: `Villainy`
- Window title: `Villainy`
- Frontend dist dir: `../dist`
- Dev server URL: `http://localhost:5173`
- Frontend dev command: `npm run dev`
- Frontend build command: `npm run build`

---

### Step 2 — Install frontend dependencies

```bash
npm install
npm install @tauri-apps/api @tauri-apps/plugin-shell
npm install -D typescript @types/react @types/react-dom vite @vitejs/plugin-react
```

---

### Step 3 — Add Rust dependencies

In `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
```

---

### Step 4 — Create the folder structure

```bash
# Frontend
mkdir -p src/components/{Library,GameDetail,DownloadManager,Settings,StoreConnections,shared}
mkdir -p src/{hooks,types,constants}

# Backend
mkdir -p src-tauri/src/{commands,cli,session,launch,download,models,utils}

# Placeholder files
touch src/types/index.ts
touch src/constants/events.ts
touch src/hooks/{useLibrary,useDownload,useLaunch,useSessions,useSettings,useEvent}.ts
touch src-tauri/src/commands/{library,install,launch,session,settings}.rs
touch src-tauri/src/cli/{legendary,gogdl,nile,lutris,mod}.rs
touch src-tauri/src/session/{store,validator,encryption}.rs
touch src-tauri/src/launch/{runner,proton,prefix,env,builder}.rs
touch src-tauri/src/download/{queue,progress}.rs
touch src-tauri/src/models/mod.rs
touch src-tauri/src/utils/{paths,process}.rs
```

---

### Step 5 — Stub `main.rs`

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod cli;
mod session;
mod launch;
mod download;
mod models;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // commands registered here as implemented
        ])
        .run(tauri::generate_context!())
        .expect("error while running Villainy");
}
```

---

### Step 6 — Stub `App.tsx`

```tsx
function App() {
    return (
        <div className="app">
            <aside className="sidebar">
                {/* Store connections, navigation */}
            </aside>
            <main className="content">
                {/* Library, GameDetail, DownloadManager */}
            </main>
        </div>
    );
}

export default App;
```

---

### Step 7 — Verify

```bash
cargo tauri dev
```

A window should open. UI is empty — that is correct. If it compiles and opens,
the scaffold is standing.

---

## 7. First Implementation Order

Work in this sequence. Each step produces something testable before the next begins.

1. `utils/paths.rs` — binary resolution for Legendary, GOGdl, Nile, Lutris
2. `cli/legendary.rs` — `list_installed()` returning parsed JSON
3. `commands/library.rs` — `get_library` command wrapping Legendary
4. `hooks/useLibrary.ts` — invoke wrapper in React
5. `components/Library/` — render the game list from real data  ← first visual milestone
6. `session/store.rs` — read/write session JSON file
7. `commands/session.rs` — `get_session_status`, `authenticate`
8. `hooks/useEvent.ts` — the listener builder
9. `download/queue.rs` — deduplication guard
10. `download/progress.rs` — stdout parsing → Tauri event pipeline
11. `launch/proton.rs` — detect Proton-GE, auto-install if missing
12. `launch/builder.rs` — launch command constructor
13. `commands/launch.rs` — `launch_game` wired end to end  ← first playable milestone
