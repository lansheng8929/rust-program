use image::{ImageBuffer, Rgba, buffer};
use my_ecs_rust::{
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};
use pixels::{Pixels, SurfaceTexture, wgpu::RequestAdapterOptions};
use winit::window::Window;

use crate::{
    EntityTrait, GameState, WINDOW_HEIGHT, WINDOW_WIDTH, animation,
    bullet::{self, Bullet},
    entity::{self, Entity},
    get_delta_time,
    gui::GuiSystem,
    player::{self, Player},
    transform::{self, Transform},
};

const FONT: [[u8; 5]; 10] = [
    [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
    [0x42, 0x61, 0x51, 0x49, 0x46], // 2
    [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39], // 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36], // 8
    [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
];

pub struct RenderSystem {
    pixels: Option<Pixels>,
}

impl System for RenderSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let delta_time = get_delta_time();
        self.clear_screen();

        if let Some(bullet_ids) = accessor.borrow_ids::<Bullet>(manager) {
            for bullet_id in bullet_ids {
                let transform = manager.borrow_component::<Transform>(*bullet_id).cloned();
                let bullet = manager.borrow_component_mut::<Bullet>(*bullet_id);

                if let (Some(transform), Some(bullet)) = (transform, bullet) {
                    if let Some(animation) = bullet.get_animation() {
                        let image_buffer = animation.get_current_frame();
                        self.draw_sprite(&transform, image_buffer);
                        (*animation).update(&delta_time);
                    }
                }
            }
        }

        if let Some(players_ids) = accessor.borrow_ids::<Player>(manager) {
            for player_id in players_ids {
                let transform = manager.borrow_component::<Transform>(*player_id).cloned();
                let player = manager.borrow_component_mut::<Player>(*player_id);

                if let (Some(transform), Some(player)) = (transform, player) {
                    if let Some(animation) = player.get_animation() {
                        let image_buffer = animation.get_current_frame();
                        self.draw_sprite(&transform, image_buffer);
                        (*animation).update(&delta_time);
                    }
                }
            }
        }

        if let Some(entity_ids) = accessor.borrow_ids::<Entity>(manager) {
            for entity_id in entity_ids {
                let transform = manager.borrow_component::<Transform>(*entity_id).cloned();
                let entity = manager.borrow_component_mut::<Entity>(*entity_id);

                if let (Some(transform), Some(entity)) = (transform, entity) {
                    if let Some(animation) = entity.get_animation() {
                        let image_buffer = animation.get_current_frame();
                        self.draw_sprite(&transform, image_buffer);
                        (*animation).update(&delta_time);
                    }
                }
            }
        }

        if let Some(game_state) = manager.get_resource::<GameState>() {
            self.draw_text(
                &game_state.score.to_string(),
                ((WINDOW_WIDTH / 2) as f32, 15.0),
            );
        }

        if self.render().is_err() {
            println!("Render failed");
        }
    }
}

impl RenderSystem {
    pub fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = pixels::PixelsBuilder::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)
            .request_adapter_options(RequestAdapterOptions {
                compatible_surface: None,
                power_preference: pixels::wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .build()
            .unwrap();

        RenderSystem {
            pixels: Some(pixels),
        }
    }

    fn draw_sprite(
        &mut self,
        transform: &Transform,
        image_buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            let (pos_x, pos_y) = (transform.position.0, transform.position.1);
            let (width, height) = image_buffer.dimensions();

            let base_angle = (1.0f32).atan2(0.0);
            let current_angle = transform.direction.1.atan2(transform.direction.0);
            let angle = current_angle - base_angle;

            let cos_angle = angle.cos();
            let sin_angle = angle.sin();

            let half_width = width as f32 / 2.0;
            let half_height = height as f32 / 2.0;

            for y in 0..height {
                for x in 0..width {
                    let pixel = image_buffer.get_pixel(x, y);

                    // Skip transparent pixels
                    if pixel.0[3] == 0 {
                        continue;
                    }

                    let translated_x = x as f32 - half_width;
                    let translated_y = y as f32 - half_height;

                    let rotated_x = translated_x * cos_angle - translated_y * sin_angle;
                    let rotated_y = translated_x * sin_angle + translated_y * cos_angle;

                    let frame_x = pos_x + rotated_x + half_width;
                    let frame_y = pos_y + rotated_y + half_height;

                    if frame_x >= 0.0
                        && frame_x < WINDOW_WIDTH as f32
                        && frame_y >= 0.0
                        && frame_y < WINDOW_HEIGHT as f32
                    {
                        let idx = ((frame_y as u32 * WINDOW_WIDTH + frame_x as u32) * 4) as usize;
                        frame[idx..idx + 4].copy_from_slice(&pixel.0);
                    }
                }
            }
        }
    }

    fn clear_screen(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = 0x00; // R
                pixel[1] = 0x00; // G
                pixel[2] = 0x00; // B
                pixel[3] = 0xff; // A
            }
        }
    }

    fn draw_text(&mut self, text: &str, pos: (f32, f32)) {
        let mut x = pos.0 as u32;
        let y = pos.1 as u32;
        for c in text.chars() {
            if let Some(digit) = c.to_digit(10) {
                self.draw_char(digit as usize, x, y);
                x += 6;
            }
        }
    }

    fn draw_char(&mut self, index: usize, x: u32, y: u32) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            let char_data = FONT[index];

            for (row, &bits) in char_data.iter().enumerate() {
                for col in 0..8 {
                    if (bits >> col) & 1 != 0 {
                        let px = x + row as u32;
                        let py = y + col as u32;

                        if px < WINDOW_WIDTH && py < WINDOW_HEIGHT {
                            let idx = ((py * WINDOW_WIDTH + px) * 4) as usize;
                            frame[idx..idx + 4].copy_from_slice(&[255, 255, 255, 255]);
                        }
                    }
                }
            }
        }
    }

    fn render(&mut self) -> Result<(), pixels::Error> {
        if let Some(pixels) = &mut self.pixels {
            pixels.render()
        } else {
            Ok(())
        }
    }
}
