use image::{DynamicImage, ImageBuffer, RgbaImage};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub angle: f32,
    pub texture: Option<RgbaImage>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            angle: 0.0,
            texture: None,
        }
    }
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            angle: 0.0,
            texture: None,
        }
    }

    pub fn load_texture(&mut self, path: &str) {
        match image::open(path) {
            Ok(img) => {
                // 将图片调整为rectangle的大小
                let resized = img.resize_exact(
                    self.width,
                    self.height,
                    image::imageops::FilterType::Nearest,
                );
                self.texture = Some(resized.to_rgba8());
            }
            Err(e) => {
                println!("Failed to load texture: {}", e);

                let mut texture = ImageBuffer::new(self.width, self.height);
                // 填充紫色 (R:255, G:0, B:255, A:255)
                for pixel in texture.pixels_mut() {
                    *pixel = image::Rgba([255, 0, 255, 255]);
                }
                self.texture = Some(texture);
            }
        }
    }

    pub fn draw_pixel(&self, x: i32, y: i32) -> [u8; 4] {
        if let Some(texture) = &self.texture {
            // 计算相对于矩形左上角的坐标
            let rel_x = x - self.x;
            let rel_y = y - self.y;

            if rel_x >= 0 && rel_y >= 0 && rel_x < self.width as i32 && rel_y < self.height as i32 {
                // 获取纹理中对应位置的像素
                let pixel = texture.get_pixel(rel_x as u32, rel_y as u32);
                return pixel.0;
            }
        }
        [0, 0, 0, 0] // 返回透明像素
    }

    pub fn is_overlapping(&self, other: &Rectangle) -> bool {
        // 计算每个矩形的边界
        let self_right = self.x + self.width as i32;
        let self_top = self.y + self.height as i32;
        let other_right = other.x + other.width as i32;
        let other_top = other.y + other.height as i32;

        !(self_right <= other.x
            || self.x >= other_right
            || self_top <= other.y
            || self.y >= other_top)
    }

    pub fn contains_point(&self, point_x: i32, point_y: i32) -> bool {
        point_x >= self.x
            && point_x < self.x + self.width as i32
            && point_y >= self.y
            && point_y < self.y + self.height as i32
    }

    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }
}
