use std::f32::consts::PI;

use image::{DynamicImage, ImageBuffer, RgbaImage};
use std::hash::Hash;

use crate::{Assets, animation::AnimationController};

#[derive(Debug, Clone)]
pub struct Rectangle<S: Clone + Eq + Hash> {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    pub angle: f32,
    pub animation: AnimationController<S>,
    pub cos_angle: f32,
    pub sin_angle: f32,
}

impl<S: Clone + Eq + Hash> Default for Rectangle<S> {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100,
            height: 100,
            angle: 0.0,
            animation: AnimationController::new(),
            cos_angle: 1.0,
            sin_angle: 0.0,
        }
    }
}

impl<S: Clone + Eq + Hash> Rectangle<S> {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            angle: 0.0,
            animation: AnimationController::new(),
            cos_angle: 1.0,
            sin_angle: 0.0,
        }
    }

    /// 绘制矩形的指定像素点
    ///
    /// # Arguments
    /// * `x` - 要绘制的x坐标
    /// * `y` - 要绘制的y坐标
    ///
    /// # Returns
    /// 返回该点的RGBA颜色值
    pub fn draw(&self, x: f32, y: f32) -> [u8; 4] {
        let rel_x = x - self.x;
        let rel_y = y - self.y;

        if rel_x >= 0.0 && rel_y >= 0.0 && rel_x < self.width as f32 && rel_y < self.height as f32 {
            if let Some(texture) = self.animation.get_current_frame() {
                let tex_width = texture.width();
                let tex_height = texture.height();

                if rel_x >= tex_width as f32 || rel_y >= tex_height as f32 {
                    return [0, 0, 0, 0];
                }
                let pixel = texture.get_pixel(rel_x as u32, rel_y as u32);
                if pixel.0[3] == 0 {
                    return [0, 0, 0, 0];
                }
                return pixel.0;
            }
        }
        [0, 0, 0, 0]
    }

    /// 检查是否与另一个矩形重叠
    ///
    /// # Arguments
    /// * `other` - 另一个矩形
    ///
    /// # Returns
    /// 如果重叠返回true
    pub fn is_overlapping<T: Clone + Eq + Hash>(&self, other: &Rectangle<T>) -> bool {
        let self_right = self.x + self.width as f32;
        let self_top = self.y + self.height as f32;
        let other_right = other.x + other.width as f32;
        let other_top = other.y + other.height as f32;

        !(self_right <= other.x
            || self.x >= other_right
            || self_top <= other.y
            || self.y >= other_top)
    }

    /// 检查一个点是否在矩形内
    ///
    /// # Arguments
    /// * `point_x` - 点的x坐标
    /// * `point_y` - 点的y坐标
    ///
    /// # Returns
    /// 如果点在矩形内返回true
    pub fn contains_point(&self, point_x: f32, point_y: f32) -> bool {
        let dx = point_x - self.x - (self.width / 2) as f32;
        let dy = point_y - self.y - (self.height / 2) as f32;

        let rotated_x = dx * self.cos_angle + dy * self.sin_angle;
        let rotated_y = -dx * self.sin_angle + dy * self.cos_angle;

        rotated_x >= -(self.width as f32 / 2.0)
            && rotated_x < (self.width / 2) as f32
            && rotated_y >= -(self.height as f32 / 2.0)
            && rotated_y < (self.height / 2) as f32
    }

    /// 检查矩形是否超出屏幕边界
    ///
    /// # Arguments
    /// * `screen_width` - 屏幕宽度
    /// * `screen_height` - 屏幕高度
    ///
    /// # Returns
    /// 如果超出边界返回true
    pub fn is_out_of_bounds(&self, screen_width: u32, screen_height: u32) -> bool {
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

    /// 设置矩形的旋转角度
    ///
    /// # Arguments
    /// * `angle` - 旋转角度(弧度)
    pub fn set_angle(&mut self, angle: f32) {
        let fixed_angle = (PI / 2.0) + angle;
        self.angle = fixed_angle;
        self.cos_angle = fixed_angle.cos();
        self.sin_angle = fixed_angle.sin();
    }

    pub fn load_animation_state(&mut self, state: S, base_path: &str, frame_count: usize) {
        self.animation
            .load_state(self.width, self.height, state, base_path, frame_count);
    }
}
