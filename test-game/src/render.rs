use ecs_rust::{
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};
use image::{ImageBuffer, Rgba, buffer};
use pixels::{Pixels, SurfaceTexture, wgpu::RequestAdapterOptions};
use winit::window::Window;

use crate::{
    EntityTrait, WINDOW_HEIGHT, WINDOW_WIDTH, animation,
    entity::{self, Entity},
    get_delta_time,
    player::{self, Player},
    transform::{self, Transform},
};

pub struct RenderSystem {
    pixels: Option<Pixels>,
}

impl System for RenderSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let delta_time = get_delta_time();
        println!("RenderSystem delta_time: {}", delta_time);
        self.clear_screen();

        if let Some(players_ids) = accessor.borrow_ids::<Player>(manager) {
            for player_id in players_ids {
                let mut position = (0, 0);

                if let Some(transform) = manager.borrow_component::<Transform>(*player_id) {
                    position = transform.position;
                }

                if let Some(player) = manager.borrow_component_mut::<Player>(*player_id) {
                    if let Some(animation) = player.get_animation() {
                        let image_buffer = animation.get_current_frame();
                        self.draw_sprite(&position, image_buffer);
                        (*animation).update(&delta_time);
                    }
                }
            }
        }

        if let Some(entity_ids) = accessor.borrow_ids::<Entity>(manager) {
            for entity_id in entity_ids {
                let mut position = (0, 0);

                if let Some(transform) = manager.borrow_component::<Transform>(*entity_id) {
                    position = transform.position;
                }

                if let Some(entity) = manager.borrow_component_mut::<Entity>(*entity_id) {
                    if let Some(animation) = entity.get_animation() {
                        let image_buffer = animation.get_current_frame();
                        self.draw_sprite(&position, image_buffer);
                        (*animation).update(&delta_time);
                    }
                }
            }
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
        position: &(i32, i32),
        image_buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            let (pos_x, pos_y) = position;
            let (width, height) = image_buffer.dimensions();

            for y in 0..height {
                for x in 0..width {
                    let pixel = image_buffer.get_pixel(x, y);
                    let frame_x = pos_x + x as i32;
                    let frame_y = pos_y + y as i32;

                    if frame_x >= 0
                        && frame_x < WINDOW_WIDTH as i32
                        && frame_y >= 0
                        && frame_y < WINDOW_HEIGHT as i32
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

    fn render(&mut self) -> Result<(), pixels::Error> {
        if let Some(pixels) = &mut self.pixels {
            pixels.render()
        } else {
            Ok(())
        }
    }
}
