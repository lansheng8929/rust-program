use crate::{gui::font8x8::FONT8X8, rectangle::Rectangle};

pub struct Label {
    pub bounds: Rectangle,
}

impl Label {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            bounds: Rectangle::new(x, y, width, height),
        }
    }
}
impl Label {
    pub fn draw(&self, frame: &mut [u8], width: usize, height: usize, label: String) {
        const FONT_8X8: [[u8; 8]; 128] = FONT8X8;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % width) as i32;
            let y = (i / width) as i32;

            if x >= self.bounds.x
                && x < self.bounds.x + self.bounds.width as i32
                && y >= self.bounds.y
                && y < self.bounds.y + self.bounds.height as i32
            {
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
    }
}
