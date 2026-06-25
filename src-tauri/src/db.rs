use rusqlite::{Connection, params, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameStats {
    pub id: i32,
    pub game_name: String,
    pub platform: String,
    pub total_playtime_minutes: i32,
    pub last_played: String,
    pub rank: i32,
    pub rating: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserStats {
    pub total_playtime_minutes: i32,
    pub games_played: i32,
    pub favorite_game: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> SqliteResult<Self> {
        let db_path = get_db_path();
        std::fs::create_dir_all(db_path.parent().unwrap()).ok();

        let conn = Connection::open(&db_path)?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    pub fn update_game_rating(&self, game_name: &str, platform: &str, rating: i32) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE game_stats SET rating = ?1 WHERE game_name = ?2 AND platform = ?3",
            params![rating, game_name, platform],
        )?;
        Ok(())
    }

    pub fn save_setting(&self, key: &str, value: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> rusqlite::Result<String> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let value = stmt.query_row(rusqlite::params![key], |row| row.get(0)).unwrap_or_default();
        Ok(value)
    }

    fn init_tables(&self) -> SqliteResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS game_stats (
                id INTEGER PRIMARY KEY,
                game_name TEXT NOT NULL,
                platform TEXT NOT NULL,
                total_playtime_minutes INTEGER DEFAULT 0,
                last_played TEXT,
                rank INTEGER DEFAULT 0,
                rating INTEGER DEFAULT 0,
                UNIQUE(game_name, platform)
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY,
                platform TEXT NOT NULL UNIQUE,
                authenticated INTEGER DEFAULT 0,
                last_login TEXT,
                token_hash TEXT
            );

            CREATE TABLE IF NOT EXISTS play_sessions (
                id INTEGER PRIMARY KEY,
                game_name TEXT NOT NULL,
                platform TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT,
                duration_minutes INTEGER
            );",
        )?;

        Ok(())
    }

    pub fn add_or_update_game_stats(
        &self,
        game_name: &str,
        platform: &str,
    ) -> SqliteResult<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO game_stats (game_name, platform, total_playtime_minutes, rank)
             VALUES (?1, ?2, 0, 0)",
            params![game_name, platform],
        )?;

        Ok(())
    }

pub fn record_play_session(
    &self,
    game_name: &str,
    platform: &str,
    duration_minutes: i32,
) -> SqliteResult<()> {
    let now = chrono::Local::now().to_rfc3339();

    // Ensure game exists in stats
    self.conn.execute(
        "INSERT OR IGNORE INTO game_stats (game_name, platform, total_playtime_minutes, rank)
         VALUES (?1, ?2, 0, 0)",
        params![game_name, platform],
    )?;

    // Add time to existing total
    self.conn.execute(
        "UPDATE game_stats 
         SET total_playtime_minutes = total_playtime_minutes + ?1,
             last_played = ?2
         WHERE game_name = ?3 AND platform = ?4",
        params![duration_minutes, now, game_name, platform],
    )?;

    Ok(())
}

    pub fn get_all_game_stats(&self) -> SqliteResult<Vec<GameStats>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, game_name, platform, total_playtime_minutes, last_played, rank, rating 
             FROM game_stats ORDER BY total_playtime_minutes DESC",
        )?;

        let stats = stmt.query_map([], |row| {
            Ok(GameStats {
                id: row.get(0)?,
                game_name: row.get(1)?,
                platform: row.get(2)?,
                total_playtime_minutes: row.get(3)?,
                last_played: row.get(4).unwrap_or_default(),
                rank: row.get(5)?,
                rating: row.get(6)?, // Must match the SELECT index
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;

        Ok(stats)
    }

    pub fn update_game_rank(&self, game_name: &str, platform: &str, rank: i32) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE game_stats SET rank = ?1 WHERE game_name = ?2 AND platform = ?3",
            params![rank, game_name, platform],
        )?;

        Ok(())
    }

}

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".villainy")
        .join("villainy.db")
}
