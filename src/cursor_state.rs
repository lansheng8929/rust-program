#[derive(Default)]
pub struct CursorState {
    pub position: (f32, f32),
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
        }
    }
}
