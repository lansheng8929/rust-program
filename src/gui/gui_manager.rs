use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::{
    WIDTH,
    cursor_state::CursorState,
    game_data::{self, GameData},
};

use super::Label;

pub struct GuiManager {
    label: Label,
}

impl GuiManager {
    pub fn new() -> Self {
        Self {
            label: Label::new(WIDTH as i32 / 2, 10, 50, 20),
        }
    }

    pub fn draw(&self, frame: &mut [u8], width: usize, height: usize, game_data: &GameData) {
        self.label
            .draw(frame, width, height, game_data.score.to_string());
    }
}
