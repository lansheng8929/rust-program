use crate::{gui::font8x8::FONT8X8, rectangle::Rectangle};

const FONT_8X8: [[u8; 8]; 128] = FONT8X8;

pub struct Label {
    pub bounds: Rectangle,
    pub label: Option<String>,
}

impl Label {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
            label: None,
        }
    }
}
impl Label {
    pub fn draw(&self, pixel: &mut [u8], x: i32, y: i32) {
        if x >= self.bounds.x
            && x < self.bounds.x + self.bounds.width as i32
            && y >= self.bounds.y
            && y < self.bounds.y + self.bounds.height as i32
        {
            let label = self.label.as_ref().unwrap();
            let text_x = x - self.bounds.x;
            let text_y = y - self.bounds.y;

            if text_y >= 0 && text_y < 8 && text_x < (label.len() * 8) as i32 {
                let char_index = text_x / 8;
                let char_x = text_x % 8;

                if let Some(c) = label.chars().nth(char_index as usize) {
                    let ascii = c as u8;
                    let pattern = FONT_8X8[ascii as usize];
                    // 修改位移方向：移除 7 - char_x
                    if (pattern[text_y as usize] >> char_x) & 1 == 1 {
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
