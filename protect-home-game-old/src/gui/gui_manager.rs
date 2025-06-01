use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::{
    WIDTH,
    game_data::{self, GameData},
};

use super::Label;

pub struct GuiManager {
    pub label: Label,
    pub label2: Label,
}

impl GuiManager {
    pub fn new() -> Self {
        Self {
            label: Label::new(WIDTH as f32 / 2.0, 10.0, 50, 20),
            label2: Label::new(WIDTH as f32 - 50.0, 10.0, 50, 20),
        }
    }

    pub fn update(&mut self, game_data: &GameData) {
        self.label.set_label(game_data.score.to_string());
        self.label2.set_label(game_data.fps.to_string());
    }

    pub fn draw(&self, pixel: &mut [u8], x: f32, y: f32) {
        self.label.draw(pixel, x, y);
        self.label2.draw(pixel, x, y);
    }
}
