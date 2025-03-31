use image::{ImageBuffer, RgbaImage};
use std::collections::HashMap;
use std::hash::Hash;

use crate::Assets;

#[derive(Debug, Clone)]
pub struct AnimationController<S: Clone + Eq + Hash> {
    states: HashMap<S, Vec<RgbaImage>>,
    current_state: Option<S>,
    current_frame: usize,
    timer: u32,
    speed: u32,
}

impl<S: Clone + Eq + Hash> Default for AnimationController<S> {
    fn default() -> Self {
        Self {
            states: HashMap::new(),
            current_state: None,
            current_frame: 0,
            timer: 0,
            speed: 10,
        }
    }
}

impl<S: Clone + Eq + Hash> AnimationController<S> {
    pub fn new() -> Self {
        Self::default()
    }

    /// 加载动画状态的帧序列
    /// - state: 动画状态
    /// - base_path: 帧图片的基础路径
    /// - frame_count: 帧数量
    pub fn load_state(&mut self, state: S, base_path: &str, frame_count: usize) {
        let mut frames = Vec::new();

        for i in 0..frame_count {
            let path = format!("{}_{}.png", base_path, i);
            if let Some(file) = Assets::get(&path) {
                if let Ok(img) =
                    image::load_from_memory_with_format(&file.data, image::ImageFormat::Png)
                {
                    frames.push(img.to_rgba8());
                }
            }
        }

        if frames.is_empty() {
            let texture = ImageBuffer::from_fn(1, 1, |_, _| image::Rgba([255, 0, 255, 255]));
            frames.push(texture);
        }

        self.states.insert(state, frames);
    }

    /// 切换动画状态
    /// - state: 要切换到的新状态
    pub fn set_state(&mut self, state: S) {
        if self.current_state.as_ref() != Some(&state) {
            self.current_state = Some(state);
            self.reset();
        }
    }

    /// 设置动画播放速度
    /// - speed: 每帧持续的时间(tick数)
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    /// 更新动画状态，处理帧切换
    pub fn update(&mut self) {
        if let Some(state) = &self.current_state {
            if let Some(frames) = self.states.get(state) {
                if !frames.is_empty() {
                    self.timer += 1;
                    if self.timer >= self.speed {
                        self.timer = 0;
                        self.current_frame = (self.current_frame + 1) % frames.len();
                    }
                }
            }
        }
    }

    /// 重置动画到初始状态
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.timer = 0;
    }

    /// 获取当前帧的图像
    /// 返回当前动画帧的图像引用
    pub fn get_current_frame(&self) -> Option<&RgbaImage> {
        self.current_state
            .as_ref()
            .and_then(|state| self.states.get(state))
            .and_then(|frames| frames.get(self.current_frame))
    }
}
