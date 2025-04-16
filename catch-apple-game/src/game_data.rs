pub struct GameData {
    pub score: i32,
}

impl Default for GameData {
    fn default() -> Self {
        GameData { score: 0 }
    }
}

impl GameData {
    pub fn new() -> Self {
        Self { score: 0 }
    }
}
