use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlaySession {
    pub game_name: String,
    pub platform: String,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameRanking {
    pub rank: usize,
    pub game_name: String,
    pub platform: String,
    pub playtime_hours: f32,
}

pub struct StatsTracker {
    current_session: Option<PlaySession>,
}

impl StatsTracker {
    pub fn new() -> Self {
        StatsTracker {
            current_session: None,
        }
    }

    pub fn start_session(&mut self, game_name: &str, platform: &str) -> Result<(), String> {
        if self.current_session.is_some() {
            return Err("A session is already active".to_string());
        }

        self.current_session = Some(PlaySession {
            game_name: game_name.to_string(),
            platform: platform.to_string(),
            start_time: SystemTime::now(),
            end_time: None,
        });

        Ok(())
    }

    pub fn end_session(&mut self) -> Result<PlaySession, String> {
        let mut session = self
            .current_session
            .take()
            .ok_or("No active session")?;

        session.end_time = Some(SystemTime::now());
        Ok(session)
    }

    pub fn get_duration_minutes(&self, session: &PlaySession) -> u32 {
        if let (Some(end), start) = (session.end_time, session.start_time) {
            let duration = end
                .duration_since(start)
                .unwrap_or_default()
                .as_secs() / 60;
            duration as u32
        } else {
            0
        }
    }

    pub fn calculate_rankings(
        stats: Vec<(String, String, i32)>,
    ) -> Vec<GameRanking> {
        let mut rankings: Vec<GameRanking> = stats
            .iter()
            .enumerate()
            .map(|(idx, (name, platform, minutes))| GameRanking {
                rank: idx + 1,
                game_name: name.clone(),
                platform: platform.clone(),
                playtime_hours: *minutes as f32 / 60.0,
            })
            .collect();

        rankings.sort_by(|a, b| b.playtime_hours.partial_cmp(&a.playtime_hours).unwrap());

        rankings
            .iter_mut()
            .enumerate()
            .for_each(|(idx, r)| r.rank = idx + 1);

        rankings
    }
}

impl Default for StatsTracker {
    fn default() -> Self {
        Self::new()
    }
}
