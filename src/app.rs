use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::apple::Apple;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::world::World;
use crate::{HEIGHT, WIDTH};

pub(crate) struct App {
    frame_count: u32,
    window: Option<Window>,
    pixels: Option<Pixels>,
    world: Option<World>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            frame_count: 0,
            window: None,
            pixels: None,
            world: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_inner_size(size)
                    .with_min_inner_size(size)
                    .with_max_inner_size(size),
            )
            .unwrap();
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        self.pixels = Some(Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap());

        let mut world = World::new(WIDTH, HEIGHT);
        world.set_player(Player::new(10, 10, 10, 3));
        // world.set_enemy(Enemy::new(10, 10, 10, 10));
        self.world = Some(world);

        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let world = self.world.as_mut().unwrap();
        let player = world.player.as_mut().unwrap();
        let pixels = self.pixels.as_mut().unwrap();

        match event {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let pressed = event.state.is_pressed();
                if let PhysicalKey::Code(key) = event.physical_key {
                    if key == KeyCode::Escape {
                        event_loop.exit();
                        return;
                    }
                    player.input(key, pressed);
                }
            }

            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                self.frame_count = (self.frame_count + 1) % u32::MAX;

                world.update();
                if self.frame_count % 60 == 0 {
                    if world.apple.len() < 100 {
                        let mut rng = rand::thread_rng();
                        world.add_apple(Apple::new(20, rng.gen_range(0..WIDTH) as i32, 0, 1));
                    }
                }

                let frame = pixels.frame_mut();
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as i32;
                    let y = (i / HEIGHT as usize) as i32;

                    world.draw(pixel, x, y);
                }
                if let Err(err) = pixels.render() {
                    println!("pixels.render() failed: {}", err);
                    event_loop.exit();
                    return;
                }

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
