use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex};

use crate::Assets;

#[derive(Eq, Hash, PartialEq)]
pub enum SoundEffect {
    SCORE = 0,
    SHOOT = 1,
}

pub struct SoundSystem {
    sound_map: HashMap<SoundEffect, String>,
    stream: Option<OutputStream>,
    stream_handle: Option<Arc<rodio::OutputStreamHandle>>,
}

impl SoundSystem {
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
        sound_map.insert(SoundEffect::SCORE, String::from("sounds/score.wav"));
        sound_map.insert(SoundEffect::SHOOT, String::from("sounds/shoot.wav"));

        Self {
            sound_map,
            stream,
            stream_handle,
        }
    }

    pub fn play_sound(&self, effect: &SoundEffect, volume: Option<f32>) {
        let path = self.sound_map.get(effect).unwrap();

        if let Some(stream_handle) = &self.stream_handle {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                if let Some(file_content) = Assets::get(path) {
                    let cursor = std::io::Cursor::new(file_content.data);
                    if let Ok(source) = Decoder::new(cursor) {
                        sink.set_volume(volume.unwrap_or(1.0));
                        sink.append(source);
                        sink.detach();
                    }
                }
            }
        }
    }
}
