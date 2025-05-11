use image::{ImageBuffer, Rgba};

use crate::Assets;

pub fn get_assets_image_buffer(
    path: &str,
    width: u32,
    height: u32,
    frame_count: usize,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut frames = Vec::new();

    for i in 0..frame_count {
        let path = format!("{}_{}.png", path, i);
        if let Some(file) = Assets::get(&path) {
            if let Ok(img) =
                image::load_from_memory_with_format(&file.data, image::ImageFormat::Png)
            {
                let mut rgba_img = img.to_rgba8();
                // 如果图片尺寸与矩形不匹配，进行缩放
                if rgba_img.width() != width || rgba_img.height() != height {
                    rgba_img = image::imageops::resize(
                        &rgba_img,
                        width,
                        height,
                        image::imageops::FilterType::Nearest,
                    );
                }
                frames.push(rgba_img);
            }
        }
    }

    if frames.is_empty() {
        let texture = ImageBuffer::from_fn(width, height, |_, _| image::Rgba([255, 0, 255, 255]));
        frames.push(texture);
    }

    frames
}
