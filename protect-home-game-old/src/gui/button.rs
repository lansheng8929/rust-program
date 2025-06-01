use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::{cursor_state::CursorState, game_data::GameData, rectangle::Rectangle};

pub struct Button {
    pub bounds: Rectangle,
    label: String,
    is_pressed: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, width: u32, height: u32, label: String) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            label,
            is_pressed: false,
        }
    }
}
impl Button {
    fn draw(&self, frame: &mut [u8], width: usize, height: usize, game_data: &mut GameData) {
        // 渲染按钮
        for i in 0..self.bounds.width {
            for j in 0..self.bounds.height {
                let x = self.bounds.x + i as f32;
                let y = self.bounds.y + j as f32;
                if x >= 0.0 && x < width as f32 && y >= 0.0 && y < height as f32 {
                    let index = (y as usize * width + x as usize) * 4;
                    if self.is_pressed {
                        frame[index..index + 4].copy_from_slice(&[255, 0, 0, 255]); // 红色
                    } else {
                        frame[index..index + 4].copy_from_slice(&[0, 255, 0, 255]); // 绿色
                    }
                }
            }
        }
    }
    fn handle_event(
        &mut self,
        button: &MouseButton,
        state: &ElementState,
        cursor_state: &CursorState,
        game_data: &mut GameData,
    ) {
        let (x, y) = cursor_state.position;
        if x >= self.bounds.x
            && x < self.bounds.x + self.bounds.width as f32
            && y >= self.bounds.y
            && y < self.bounds.y + self.bounds.height as f32
        {
            if button == &MouseButton::Left && state == &ElementState::Pressed {
                self.is_pressed = true;
            } else if button == &MouseButton::Left && state == &ElementState::Released {
                self.is_pressed = false;
            }
        }
    }
}
