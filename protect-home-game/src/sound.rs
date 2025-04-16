use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use crate::Assets;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum SoundEffect {
    Background,
    Collect,
    GameOver,
}

pub struct SoundManager {
    sound_map: HashMap<SoundEffect, String>,
    stream: Option<OutputStream>,
    stream_handle: Option<Arc<rodio::OutputStreamHandle>>,
    background_sink: Option<Sink>,
    volume: f32,
}

impl Default for SoundManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundManager {
    pub fn new() -> Self {
        // 创建音频输出流
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(Arc::new(h))),
            Err(e) => {
                eprintln!("Failed to initialize audio: {}", e);
                (None, None)
            }
        };

        // 初始化音效映射
        let mut sound_map = HashMap::new();
        sound_map.insert(
            SoundEffect::Background,
            String::from("sounds/background.wav"),
        );
        sound_map.insert(SoundEffect::Collect, String::from("sounds/collect.wav"));
        sound_map.insert(SoundEffect::GameOver, String::from("sounds/gameover.wav"));

        Self {
            sound_map,
            stream,
            stream_handle,
            background_sink: None,
            volume: 1.0,
        }
    }

    // 添加新的音效
    pub fn add_sound(&mut self, effect: SoundEffect, path: String) {
        self.sound_map.insert(effect, path);
    }

    // 播放一次性音效
    pub fn play_sound(&self, effect: &SoundEffect) -> Result<(), String> {
        let path = self.sound_map.get(effect).ok_or("Sound effect not found")?;

        if let Some(stream_handle) = &self.stream_handle {
            match Sink::try_new(&stream_handle) {
                Ok(sink) => {
                    let file_content = Assets::get(path)
                        .ok_or_else(|| format!("Failed to load embedded sound file: {}", path))?;
                    let cursor = std::io::Cursor::new(file_content.data);
                    match Decoder::new(cursor) {
                        Ok(source) => {
                            sink.set_volume(self.volume);
                            sink.append(source);
                            sink.detach();
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to decode audio: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to create audio sink: {}", e)),
            }
        } else {
            Err("Audio stream not initialized".to_string())
        }
    }

    // 播放背景音乐
    pub fn play_background_music(&mut self) -> Result<(), String> {
        let path = self
            .sound_map
            .get(&SoundEffect::Background)
            .ok_or("Background music not found")?;

        if let Some(stream_handle) = &self.stream_handle {
            match Sink::try_new(&stream_handle) {
                Ok(sink) => match File::open(path) {
                    Ok(file) => {
                        let buf_reader = BufReader::new(file);
                        match Decoder::new(buf_reader) {
                            Ok(source) => {
                                sink.set_volume(self.volume);
                                sink.append(source.repeat_infinite());
                                self.background_sink = Some(sink);
                                Ok(())
                            }
                            Err(e) => Err(format!("Failed to decode audio: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Failed to open background music: {}", e)),
                },
                Err(e) => Err(format!("Failed to create audio sink: {}", e)),
            }
        } else {
            Err("Audio stream not initialized".to_string())
        }
    }

    // 停止背景音乐
    pub fn stop_background_music(&mut self) {
        if let Some(sink) = &self.background_sink {
            sink.stop();
        }
        self.background_sink = None;
    }

    // 设置全局音量
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.background_sink {
            sink.set_volume(self.volume);
        }
    }

    // 获取当前音量
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    // 暂停所有声音
    pub fn pause_all(&mut self) {
        if let Some(sink) = &self.background_sink {
            sink.pause();
        }
    }

    // 恢复所有声音
    pub fn resume_all(&mut self) {
        if let Some(sink) = &self.background_sink {
            sink.play();
        }
    }
}
