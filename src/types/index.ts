export type Store = 'epic' | 'gog' | 'amazon' | 'sideload';

export interface Game {
  id: string;a
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
