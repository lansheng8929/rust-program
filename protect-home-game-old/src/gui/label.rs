use crate::{gui::font8x8::FONT8X8, rectangle::Rectangle};

const FONT_8X8: [[u8; 8]; 128] = FONT8X8;

pub struct Label {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    pub label: Option<String>,
}

impl Label {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            label: None,
        }
    }
}
impl Label {
    pub fn draw(&self, pixel: &mut [u8], x: f32, y: f32) {
        if x >= self.x
            && x < self.x + self.width as f32
            && y >= self.y
            && y < self.y + self.height as f32
        {
            let label = self.label.as_ref().unwrap();
            let text_x = x - self.x;
            let text_y = y - self.y;

            if text_y >= 0.0 && text_y < 8.0 && text_x < (label.len() * 8) as f32 {
                let char_index = text_x / 8.0;
                let char_x = text_x % 8.0;

                if let Some(c) = label.chars().nth(char_index as usize) {
                    let ascii = c as u8;
                    let pattern = FONT_8X8[ascii as usize];
                    if (pattern[text_y as usize] >> (char_x as u8)) & 1 == 1 {
                        pixel.copy_from_slice(&[255, 255, 255, 255]);
                    }
                }
            }
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }
}
