use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::apple::Apple;
use crate::cursor_state::CursorState;
use crate::enemy::Enemy;
use crate::game_data::GameData;
use crate::gui::{Button, GuiManager, Label};
use crate::player::Entity;
use crate::world::World;
use crate::{HEIGHT, WIDTH};

pub struct App {
    frame_count: u32,
    window: Option<Window>,
    pixels: Option<Pixels>,
    world: Option<World>,
    gui: Option<GuiManager>,
    cursor_state: CursorState,
    game_data: GameData,
}

impl Default for App {
    fn default() -> Self {
        Self {
            frame_count: 0,
            window: None,
            pixels: None,
            world: None,
            gui: None,
            cursor_state: CursorState::new(),
            game_data: GameData::default(),
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
        world.set_player(Entity::new(20, 10, HEIGHT as i32 - 10, 3));
        // world.set_enemy(Enemy::new(10, 10, 10, 10));
        self.world = Some(world);

        let gui = GuiManager::new();
        self.gui = Some(gui);

        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let world = self.world.as_mut().unwrap();
        let player = world.player.as_mut().unwrap();
        let pixels = self.pixels.as_mut().unwrap();
        let gui = self.gui.as_mut().unwrap();
        let cursor_state = &mut self.cursor_state;

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let window = self.window.as_ref().unwrap();
                let window_size = window.inner_size();

                // 计算缩放比例
                let scale_x = WIDTH as f64 / window_size.width as f64;
                let scale_y = HEIGHT as f64 / window_size.height as f64;

                // 转换坐标
                let pixels_x = (position.x * scale_x) as i32;
                let pixels_y = (position.y * scale_y) as i32;

                cursor_state.position = (pixels_x, pixels_y);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state: _,
                button: _,
            } => {
                // gui.handle_event(&button, &state, &cursor_state);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
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

                world.update(&mut self.game_data);
                gui.update(&self.game_data);

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
                    gui.draw(pixel, x, y);
                }
                if let Err(err) = pixels.render() {
                    println!("pixels.render() failed: {}", err);
                    event_loop.exit();
                    return;
                }

                // Draw.

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
