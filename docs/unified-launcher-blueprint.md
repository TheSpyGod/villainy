# Unified Linux Game Launcher — Project Blueprint

**Author:** Gabe  
**Date:** May 2026  
**Status:** Planning Phase

---

## 1. Project Overview

A unified Linux game launcher that consolidates Heroic (Epic, GOG, Amazon) and Lutris into a single interface. The user manages their entire game library — across all stores — from one place, with Proton-GE handling Windows game compatibility.

The project consists of two distinct components:

- **The launcher app** — the core product, built with Tauri + React + TypeScript
- **The marketing and documentation website** — a full-stack web platform satisfying university requirements, built with React + Express + PostgreSQL

---

## 2. Academic Requirements Mapping

| Requirement | How It Is Satisfied |
|---|---|
| Chosen framework | React (frontend) + Express (backend) |
| Database implementation | PostgreSQL — users, reviews, download records |
| Backend logic layer | Express API handling auth, reviews, download tracking |
| Frontend layer | React web UI for the marketing/docs site |
| Full CRUD | Create/Read/Update/Delete on users, reviews, downloads |
| MVC pattern | Models via Prisma, Views via React, Controllers via Express routes |

---

## 3. Technology Stack

### Launcher App
| Layer | Technology |
|---|---|
| UI Framework | Tauri v2 |
| Frontend | React + TypeScript |
| Backend/Logic | Rust (Tauri core) |
| Compatibility Layer | Proton-GE |
| CLI Integration | Legendary, GOGdl, Nile, Lutris CLI |

### Website
| Layer | Technology |
|---|---|
| Frontend | React + TypeScript |
| Backend | Node.js + Express |
| Database | PostgreSQL |
| ORM | Prisma |

---

## 4. Understanding the Existing Ecosystem

### Heroic Games Launcher
- An Electron app written in TypeScript that wraps three separate CLI tools
- **Legendary** — handles Epic Games (auth, install, launch, cloud saves)
- **GOGdl** — handles GOG Games
- **Nile** — handles Amazon Games
- Stores config in `~/.config/heroic/config.json` and per-game settings in `~/.config/heroic/GameConfig/{game}.json`
- Launches games via protocol handler: `heroic://launch/{appId}`
- Has no public API — all integration is via CLI tools or direct file parsing

### Lutris
- Python-based launcher with a rich CLI
- Stores game library in `~/.local/share/lutris/pga.db` — an SQLite database
- Key CLI flags: `-l` list games, `-j` JSON output, `-o` installed only, `-b` generate bash launch script
- Launch via protocol: `lutris:rungameid/{id}`
- Supports multiple runners: Wine, Steam, native Linux, emulators

### Legendary (Epic CLI)
- Full command set: `auth`, `install`, `launch`, `list-games`, `list-installed`, `verify-game`, `sync-saves`
- `--wrapper` flag allows injecting Proton-GE as a compatibility wrapper
- `--dry-run` prints the exact launch command without executing — useful for testing
- `--json` flag on most commands returns structured, parseable output

### Proton-GE
- Community build of Valve's Proton compatibility layer
- Lives in `~/.steam/root/compatibilitytools.d/`
- Acts as a wrapper around the game executable — not an injector
- Requires environment variables: `STEAM_COMPAT_DATA_PATH`, `WINEPREFIX`, `STEAM_COMPAT_CLIENT_INSTALL_PATH`

---

## 5. The Five Core Pillars

### Pillar 1 — CLI Integration

The abstraction layer between the app and the underlying CLI tools. Nothing in the rest of the app touches the CLI directly.

**Components:**
- **CLI Abstraction Layer** — one Rust module per tool (Legendary, GOGdl, Nile, Lutris). Each exposes clean async functions: `list_installed()`, `launch_game(id)`, `install_game(id, path)`.
- **Binary Resolution** — detect binary locations at startup. Check PATH, then Flatpak paths (`/var/lib/flatpak/...`), then user-local paths. Degrade gracefully if a binary is missing.
- **Process Management** — handle stdout/stderr streaming, exit codes, timeouts, and cleanup of orphaned processes.
- **Output Parsing** — normalise each tool's output into a unified internal `Game` struct regardless of source store.
- **Error Handling** — categorise errors: auth errors trigger re-auth flow, network errors trigger retry with backoff, fatal errors surface human-readable messages. Raw stderr never reaches the user.
- **Command Queueing** — serialise install operations per store. Allow concurrent launches but not concurrent installs on the same store.

**Key insight:** Legendary being Python means it could potentially be integrated as a library rather than a subprocess. Further research needed before committing to either approach.

---

### Pillar 2 — Session Management

Handles authentication state for each connected game store, stored locally.

**Components:**
- **Per-Store Auth Flows:**
  - Epic via Legendary — browser OAuth, Legendary owns the credential in `~/.config/legendary/user.json`
  - GOG via GOGdl — same pattern, GOGdl owns its credential
  - Amazon via Nile — same pattern
  - Lutris — interacted with via CLI and `pga.db`, no direct token management
- **Session State Store** — a local JSON file (owner read/write only, `600` permissions) tracking: which stores are authenticated, username per store, last validation timestamp.
- **Session Validation on Startup** — run `legendary status` and equivalents on every app launch. Flag expired sessions and prompt re-auth before the user hits a failure mid-use.
- **Re-auth Recovery Flow** — non-destructive. A GOG session expiring does not interrupt an active Epic download.
- **Session Isolation** — each store's session is completely independent.
- **Optional Encryption** — session file can be password-protected via OS keyring integration (`keytar` or equivalent).

---

### Pillar 3 — UI Rendering

**Framework Decision: Tauri v2 + React + TypeScript**

Chosen over Slint and Electron for the following reasons:
- React/TypeScript skills transfer directly — no new UI paradigm to learn
- Uses the system's native WebKitGTK webview — significantly lighter than Electron's bundled Chromium
- Rust backend handles all process spawning, file system operations, and session management natively
- Binary size: ~3MB vs Electron's 80–120MB
- Vibrant production ecosystem

**Core Views:**
- **Library** — unified grid/list of all games across all stores, filterable by store, install status, last played
- **Game Detail** — cover art, description, source store, install/launch button, per-game settings override
- **Download Manager** — active downloads, queue, progress bars, speed, ETA
- **Settings** — global config, per-store config, Proton-GE version management, default install paths
- **Store Connections** — connect/disconnect per store, auth status
- **Search** — across the unified library

**IPC Layer:**
The React renderer communicates with the Rust backend via Tauri's typed command system. The renderer is purely presentational — it requests, the Rust backend acts. All CLI calls originate from the Rust layer.

**Real-time Feedback:**
The Rust backend streams progress events to the React frontend via Tauri events. Download progress, install progress, and game launch status all update live without polling.

---

### Pillar 4 — Download Management

An orchestration layer over the CLI tools' own download implementations. No custom HTTP download code.

**Components:**
- **Delegated Downloads** — Legendary, GOGdl, and Nile handle their own download logic. The app invokes them, monitors progress, and presents it to the user.
- **Install Path Management** — global default path and per-game override. Validate before install: directory exists, sufficient disk space, write permissions. Calculate required space from manifest before starting.
- **Progress Tracking** — pipe CLI stdout in real time, parse progress output, push to UI via Tauri events. Track: bytes downloaded, total, speed, ETA.
- **Queue Management** — max N simultaneous downloads, queue persists across restarts. Allow reorder and cancellation.
- **Pause and Resume** — exposed in UI where the underlying tool supports it (Legendary yes, others vary).
- **Post-Install Verification** — run `legendary verify-game` before marking as installed.
- **Uninstall** — delegate to CLI tool, confirm via exit code, remove from local state.

---

### Pillar 5 — Game Launching and Compatibility

The most technically complex pillar. A launch failure is the most visible failure possible.

**Components:**
- **Runner Selection Logic:**
  - Native Linux binary → run directly
  - Windows game (Epic/GOG/Amazon) → Proton-GE by default
  - Lutris-managed game → let Lutris handle the runner, or read from `pga.db` and replicate
  - All rules overridable per game

- **Proton-GE Integration:**
  - Detect installed versions in `~/.steam/root/compatibilitytools.d/`
  - Fetch new versions from GE-Proton GitHub releases API
  - Default version set globally, overridable per game
  - Injected via `--wrapper` flag on Legendary or directly in the launch command

- **Wine Prefix Management:**
  - One prefix per game, created on first launch if absent
  - Stored predictably at `~/.local/share/{appname}/prefixes/{gameId}/`
  - User can open prefix folder for manual tweaks
  - Prefix paths update if the user moves game files

- **Environment Variables:**
  - Sensible defaults for all Proton launches: `STEAM_COMPAT_DATA_PATH`, `WINEPREFIX`, `STEAM_COMPAT_CLIENT_INSTALL_PATH`
  - Global additions (e.g. MangoHud, GameMode)
  - Per-game overrides
  - Final env passed to `std::process::Command` in Rust

- **Launch Command Constructor:**
  - Takes a `Game` struct + resolved settings → outputs the exact command
  - Testable in isolation without executing a real launch

- **Pre-launch Checks:**
  - Game installed at expected path?
  - Proton-GE version available?
  - Wine prefix valid?
  - Store session active (for online games)?
  - Game already running?

- **Process Monitoring:**
  - Track PID, launch timestamp, running status
  - Show game as "running" in UI
  - Track playtime
  - Detect crashes via non-zero exit code

- **Crash Detection:**
  - Capture last N lines of stderr on non-zero exit
  - Surface human-readable error message
  - Offer raw log access for advanced users
  - Write per-game log file for persistence

- **Post-launch State:**
  - Update playtime and last-played timestamp
  - Mark game as no longer running in UI

---

## 6. Website Component

A marketing and documentation platform. Not a control panel for the launcher — a standalone web presence.

### Pages
- **Landing** — what the app is, why it exists, screenshots, feature list
- **Documentation** — install guide, configuration, usage
- **Download** — binary download, live download counter
- **Reviews** — community ratings and written reviews

### Database Schema (PostgreSQL)

```
users         — id, username, email, password_hash, created_at
reviews       — id, user_id, rating, body, created_at, updated_at
downloads     — id, version, platform, downloaded_at
```

### CRUD Coverage
| Table | C | R | U | D |
|---|---|---|---|---|
| users | Register | Profile page | Edit profile | Delete account |
| reviews | Submit review | Read reviews | Edit own review | Delete own review |
| downloads | On each download | Counter display | — | Admin only |

---

## 7. Known Open Questions

- **Legendary as library vs subprocess** — needs further investigation. Direct Python library import is cleaner but adds a Python runtime dependency.
- **Custom GOGdl implementation** — GOGdl appears to use authenticated wget calls. Feasible to reimplement, but deferred to post-MVP.
- **GOGdl/Nile session auth flows** — less documented than Legendary. Needs hands-on testing.
- **Proton-GE injection for non-Legendary games** — the `--wrapper` flag works cleanly for Legendary. GOGdl and Nile may require a different approach.

---

## 8. Next Steps

1. Set up Tauri + React + TypeScript project scaffold
2. Implement Legendary CLI abstraction module (list, install, launch)
3. Implement session state store and validation on startup
4. Implement Lutris CLI abstraction module
5. Build the basic library view in React
6. Wire up IPC between React and Rust backend
7. Implement Proton-GE detection and launch command constructor
8. Add download progress streaming
9. Begin website scaffold (React + Express + PostgreSQL)
10. Implement reviews and download counter CRUD
