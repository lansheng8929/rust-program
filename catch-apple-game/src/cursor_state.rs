#[derive(Default)]
pub struct CursorState {
    pub position: (i32, i32),
}

impl CursorState {
    pub fn new() -> Self {
        Self { position: (0, 0) }
    }
}
