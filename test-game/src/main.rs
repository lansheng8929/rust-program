use collision_box::CollisionBox;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use ecs_rust::world::World;
use pixels::wgpu::RequestAdapterOptions;
use pixels::{Pixels, SurfaceTexture};
use player::{Player, PlayerSystem};
use rand::Rng;
use transform::Transform;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod collision_box;
mod player;
mod transform;

struct RenderSystem {
    pixels: Option<Pixels>,
}
impl System for RenderSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        self.clear_screen();

        if let (Some(players), Some(transforms)) = (
            manager.borrow_components::<Player>(),
            manager.borrow_components::<Transform>(),
        ) {
            for (i, (_player, transform)) in players.iter().zip(transforms.iter()).enumerate() {
                self.draw_rectangle(
                    transform.position.0,
                    transform.position.1,
                    10,
                    10,
                    [0xff, 0x00, 0x00, 0xff],
                );
            }
        }

        if self.render().is_err() {
            println!("Render failed");
        }
    }
}

impl RenderSystem {
    fn new(window: &Window) -> Self {
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

    fn draw_rectangle(&mut self, x: i32, y: i32, width: u32, height: u32, color: [u8; 4]) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            for dy in 0..height {
                for dx in 0..width {
                    let x = x + (dx as i32);
                    let y = y + (dy as i32);

                    if x < 0 || y < 0 || x >= (WINDOW_WIDTH as i32) || y >= (WINDOW_HEIGHT as i32) {
                        continue;
                    }

                    let index = ((y * (WINDOW_WIDTH as i32) + x) * 4) as usize;
                    frame[index] = color[0]; // R
                    frame[index + 1] = color[1]; // G
                    frame[index + 2] = color[2]; // B
                    frame[index + 3] = color[3]; // A
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

#[derive(Default)]
struct App {
    window: Option<Window>,
    pixels: Option<Pixels>,
    world: Option<World>,
}

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
                    .with_title("Game Window"),
            )
            .unwrap();

        let mut world = World::new();
        world
            .register_component::<Player>()
            .register_component::<Transform>()
            .register_component::<CollisionBox>();

        let mut rng = rand::thread_rng();

        for i in 0..1 {
            let entity_id = world.create_entity();

            // 生成随机位置
            let random_x = rng.gen_range(0..WINDOW_WIDTH - 50); // 确保不会超出窗口宽度
            let random_y = rng.gen_range(0..WINDOW_HEIGHT - 50); // 确保不会超出窗口高度

            world.add_component_to_entity(entity_id, Player { name: "Player" });
            world.add_component_to_entity(
                entity_id,
                Transform {
                    position: (random_x as i32, random_y as i32),
                    velocity: (10, 10),
                },
            );
            world.add_component_to_entity(
                entity_id,
                CollisionBox {
                    width: 10,
                    height: 10,
                    is_trigger: false,
                },
            );
        }

        world
            .add_system(PlayerSystem {})
            .add_system(RenderSystem::new(&window));
        world.update();

        self.window = Some(window);
        self.world = Some(world);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
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
                    // player.input(key, pressed);
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
                if let Some(world) = &mut self.world {
                    world.update();
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

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}
