# Current Plan

## Phase 0 — Fix the scaffold

- [x] Delete `src-tauri/src/mod.rs`
- [x] Resolve `lib.rs` vs `main.rs` conflict — migrate module declarations and Builder setup into `lib.rs`, make `main.rs` just call `lib_run()`
- [x] Fix `src/types/index.ts` line 3: `id: string;a` → `id: string;`
- [x] Fix `src/hooks/useEvent.ts`: add missing `function` keyword, fix `unlisten?.{}` → `unlisten?.()`

## Phase 1 — Data model

- [x] Define core Rust structs in `models/mod.rs`: `Game`, `Store`, `Settings`, `SessionStatus`, `DownloadProgress` with `Serialize`/`Deserialize`

## Phase 2 — First real data

- [x] `utils/paths.rs` — binary resolution for Legendary, GOGdl, Nile, Lutris (PATH → Flatpak → user-local)
- [x] `cli/legendary.rs` — `list_installed()` calling `legendary list-installed --json`, parsed into `Vec<Game>`
- [x] `commands/library.rs` — thin `get_library` command wrapping the above
- [x] `hooks/useLibrary.ts` — `invoke('get_library')` wrapper
- [x] `src/components/Library/` — render the game list ← first visual milestone
- [x] `src/components/StoreConnections/` — per-store auth status and authenticate/logout buttons

## Phase 3 — Session

- [x] `session/store.rs` — read/write session JSON file (mode 600)
- [x] `session/validator.rs` — startup validation via `legendary status --json`
- [x] `commands/session.rs` — `get_session_status`, `authenticate`, `logout`, `validate_sessions`
- [x] `hooks/useSession.ts` — invoke wrappers + session event handling

## Phase 4 — Launch

- [ ] `launch/proton.rs` — detect installed Proton-GE versions in `~/.steam/root/compatibilitytools.d/`
- [ ] `launch/env.rs` — assemble environment variable set for Proton launches
- [ ] `launch/builder.rs` — launch command constructor (testable without executing)
- [ ] `launch/runner.rs` — runner selection logic (native vs Proton vs Lutris)
- [ ] `commands/launch.rs` + `hooks/useLaunch.ts` ← first playable milestone

## Phase 5 — Downloads

- [ ] `download/progress.rs` — stdout parsing → Tauri event emission
- [ ] `download/queue.rs` — deduplication + queue management
- [ ] `commands/install.rs` + `hooks/useDownload.ts`

## Phase 6 — Website

- [ ] Express + PostgreSQL + Prisma scaffold
- [ ] Auth (users table: register, login, edit profile, delete account)
- [ ] Reviews CRUD (submit, read, edit own, delete own)
- [ ] Download counter (increment on download, display count, admin delete)
