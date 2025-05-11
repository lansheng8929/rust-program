use image::{ImageBuffer, Rgba};

#[derive(Debug)]
pub struct Animation {
    pub frames: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, // 每个状态对应的动画帧
    pub current_frame: usize,                        // 当前播放的帧索引
    pub frame_duration: f32,                         // 每帧持续的时间（单位：毫秒）
    pub elapsed_time: f32,                           // 当前帧已播放的时间
}

impl Animation {
    pub fn new(frames: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, frame_duration: f32) -> Self {
        Animation {
            frames,
            current_frame: 0,
            frame_duration,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: &f32) {
        self.elapsed_time += delta_time;
        println!("elapsed_time: {}", self.elapsed_time);

        if self.elapsed_time >= self.frame_duration {
            self.elapsed_time = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    pub fn get_current_frame(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        &self.frames[self.current_frame]
    }
}
