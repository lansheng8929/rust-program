use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use ecs_rust::world::World;
use pixels::{Pixels, SurfaceTexture};
use player::Player;
use rand::Rng;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

mod player;

struct PlayerSystem;

impl System for PlayerSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        if let Some(mut players) = manager.borrow_components_mut::<Player>() {
            // 更新位置并处理边界碰撞
            for player in players.iter_mut() {
                let new_x = player.position.0 as i32 + player.velocity.0;
                let new_y = player.position.1 as i32 + player.velocity.1;

                // 边界碰撞检测和反弹
                if new_x <= 0 || new_x >= (WINDOW_WIDTH - 50) as i32 {
                    player.velocity.0 = -player.velocity.0; // 水平反弹
                } else {
                    player.position.0 = new_x as u32;
                }

                if new_y <= 0 || new_y >= (WINDOW_HEIGHT - 50) as i32 {
                    player.velocity.1 = -player.velocity.1; // 垂直反弹
                } else {
                    player.position.1 = new_y as u32;
                }
            }

            // 玩家之间的碰撞检测
            let player_count = players.len();
            for i in 0..player_count {
                let (left, right) = players.split_at_mut(i + 1);
                let player_a = &mut left[i];
                for player_b in right.iter_mut() {
                    // 检测两个玩家是否碰撞
                    if is_colliding(
                        player_a.position.0,
                        player_a.position.1,
                        10,
                        10,
                        player_b.position.0,
                        player_b.position.1,
                        10,
                        10,
                    ) {
                        // 简单的碰撞响应：反转速度
                        player_a.velocity.0 = -player_a.velocity.0;
                        player_a.velocity.1 = -player_a.velocity.1;

                        player_b.velocity.0 = -player_b.velocity.0;
                        player_b.velocity.1 = -player_b.velocity.1;
                    }
                }
            }
        }
    }
}

/// 检测两个矩形是否碰撞
fn is_colliding(x1: u32, y1: u32, w1: u32, h1: u32, x2: u32, y2: u32, w2: u32, h2: u32) -> bool {
    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}

struct RenderSystem {
    pixels: Option<Pixels>,
}

impl System for RenderSystem {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        println!("!!!!!");
        // 清空屏幕
        self.clear_screen();

        // 渲染所有游戏对象
        if let Some(players) = manager.borrow_components::<Player>() {
            for (i, player) in players.iter().enumerate() {
                // 为每个玩家绘制一个矩形
                self.draw_rectangle(
                    player.position.0,        // x 位置
                    player.position.1,        // y 位置
                    10,                       // 宽度
                    10,                       // 高度
                    [0xff, 0x00, 0x00, 0xff], // 红色
                );
            }
        }

        // 渲染到屏幕
        if self.render().is_err() {
            println!("Render failed");
        }
    }
}

impl RenderSystem {
    fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap();

        RenderSystem {
            pixels: Some(pixels),
        }
    }

    fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();
            for dy in 0..height {
                for dx in 0..width {
                    let x = x + dx;
                    let y = y + dy;

                    if x >= WINDOW_WIDTH || y >= WINDOW_HEIGHT {
                        continue;
                    }

                    let index = ((y * WINDOW_WIDTH + x) * 4) as usize;
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
        world.register_component::<Player>();

        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let entity_id = world.create_entity();

            // 生成随机位置
            let random_x = rng.gen_range(0..(WINDOW_WIDTH - 50)); // 确保不会超出窗口宽度
            let random_y = rng.gen_range(0..(WINDOW_HEIGHT - 50)); // 确保不会超出窗口高度

            world.add_component_to_entity(
                entity_id,
                Player {
                    name: "Player",
                    position: (random_x, random_y),
                    velocity: (2, 2),
                },
            );
        }
        world.add_system(PlayerSystem {});
        world.add_system(RenderSystem::new(&window));
        world.update();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
        self.world = Some(world);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
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
