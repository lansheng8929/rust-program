pub struct GameData {
    pub score: i32,
    pub fps: f32,
}

impl Default for GameData {
    fn default() -> Self {
        GameData { score: 0, fps: 0.0 }
    }
}

impl GameData {
    pub fn new() -> Self {
        Self { score: 0, fps: 0.0 }
    }
}
