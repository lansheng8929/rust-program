use std::f32::consts::PI;

use image::{DynamicImage, ImageBuffer, RgbaImage};

use crate::Assets;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    pub angle: f32,
    pub texture: Option<RgbaImage>,
    pub cos_angle: f32,
    pub sin_angle: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100,
            height: 100,
            angle: 0.0,
            texture: None,
            cos_angle: 1.0,
            sin_angle: 0.0,
        }
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            angle: 0.0,
            texture: None,
            cos_angle: 1.0,
            sin_angle: 0.0,
        }
    }

    pub fn load_texture(&mut self, path: &str) {
        if let Some(file) = Assets::get(path) {
            if let Ok(img) = image::load_from_memory(&file.data) {
                // 将图片调整为rectangle的大小
                let resized = img.resize_exact(
                    self.width,
                    self.height,
                    image::imageops::FilterType::Nearest,
                );
                self.texture = Some(resized.to_rgba8());
            }
        } else {
            println!("Failed to load texture from path: {}", path);

            let mut texture = ImageBuffer::new(self.width, self.height);
            // 填充紫色 (R:255, G:0, B:255, A:255)
            for pixel in texture.pixels_mut() {
                *pixel = image::Rgba([255, 0, 255, 255]);
            }
            self.texture = Some(texture);
        }
    }

    pub fn draw_pixel(&self, x: f32, y: f32) -> [u8; 4] {
        if let Some(texture) = &self.texture {
            // 计算相对于矩形左上角的坐标
            let rel_x = x - self.x;
            let rel_y = y - self.y;

            if rel_x >= 0.0
                && rel_y >= 0.0
                && rel_x < self.width as f32
                && rel_y < self.height as f32
            {
                // 获取纹理中对应位置的像素
                let pixel = texture.get_pixel(rel_x as u32, rel_y as u32);
                return pixel.0;
            }
        }
        [0, 0, 0, 0] // 返回透明像素
    }

    pub fn is_overlapping(&self, other: &Rectangle) -> bool {
        // 计算每个矩形的边界
        let self_right = self.x + self.width as f32;
        let self_top = self.y + self.height as f32;
        let other_right = other.x + other.width as f32;
        let other_top = other.y + other.height as f32;

        !(self_right <= other.x
            || self.x >= other_right
            || self_top <= other.y
            || self.y >= other_top)
    }

    pub fn contains_point(&self, point_x: f32, point_y: f32) -> bool {
        // point_x >= self.x
        //     && point_x < self.x + self.width as f32
        //     && point_y >= self.y
        //     && point_y < self.y + self.height as f32

        let dx = point_x - self.x - (self.width / 2) as f32;
        let dy = point_y - self.y - (self.height / 2) as f32;

        let rotated_x = dx * self.cos_angle + dy * self.sin_angle;
        let rotated_y = -dx * self.sin_angle + dy * self.cos_angle;

        rotated_x >= -(self.width as f32 / 2.0)
            && rotated_x < (self.width / 2) as f32
            && rotated_y >= -(self.height as f32 / 2.0)
            && rotated_y < (self.height / 2) as f32
    }

    pub fn is_out_of_bounds(&self, screen_width: u32, screen_height: u32) -> bool {
        // 计算旋转后的矩形的四个角点
        let half_width = self.width as f32 / 2.0;
        let half_height = self.height as f32 / 2.0;
        let center_x = self.x as f32 + half_width;
        let center_y = self.y as f32 + half_height;

        let corners = [
            (-half_width, -half_height),
            (half_width, -half_height),
            (half_width, half_height),
            (-half_width, half_height),
        ];

        // 检查旋转后的每个角点是否在屏幕范围内
        for (dx, dy) in corners.iter() {
            let rotated_x = center_x + dx * self.cos_angle - dy * self.sin_angle;
            let rotated_y = center_y + dx * self.sin_angle + dy * self.cos_angle;

            if rotated_x < 0.0
                || rotated_x >= screen_width as f32
                || rotated_y < 0.0
                || rotated_y >= screen_height as f32
            {
                return true;
            }
        }

        false
    }

    pub fn set_angle(&mut self, angle: f32) {
        let fixed_angle = (PI / 2.0) + angle;
        self.angle = fixed_angle;
        self.cos_angle = fixed_angle.cos();
        self.sin_angle = fixed_angle.sin();
    }
}
