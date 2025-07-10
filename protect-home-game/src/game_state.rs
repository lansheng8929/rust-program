use std::time::Instant;

#[derive(Debug, Clone)]
pub struct GameState {
    pub delta_time: f32,
    pub last_time: Option<Instant>,
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            last_time: None,
            score: 0,
        }
    }
}

impl GameState {}
