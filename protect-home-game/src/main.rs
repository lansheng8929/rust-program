use std::collections::HashMap;
use std::time::Instant;

use animation::Animation;
use bullet::{Bullet, BulletState, BulletSystem};
use collision_box::{CollisionBox, CollisionSystem};
use entity::{Entity, EntityState, EntitySystem};
use gui::GuiSystem;
use input::{Input};
use game_state::GameState;
use my_ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use my_ecs_rust::system::System;
use my_ecs_rust::world::World;
use pixels::wgpu::RequestAdapterOptions;
use pixels::{Pixels, SurfaceTexture};
use player::{Player, PlayerSystem};
use render::RenderSystem;
use rust_embed::RustEmbed;
use sound::SoundSystem;
use transform::Transform;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod animation;
mod bullet;
mod collision_box;
mod entity;
mod gui;
mod input;
mod player;
mod render;
mod sound;
mod transform;
mod utils;
mod game_state;

use std::sync::{LazyLock, Mutex};



#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub trait EntityTrait<T> {
    fn set_state(&mut self, state: T);
    fn get_animation(&mut self) -> Option<&mut Animation>;
}

#[derive(Default)]
struct App {
    window: Option<Window>,
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

        world.add_resource::<SoundSystem>(SoundSystem::new());
        world.add_resource::<GameState>(GameState::default());
        world.add_resource::<Input>(Input::default());

        world
            .register_component::<Entity>()
            .register_component::<Player>()
            .register_component::<Transform>()
            .register_component::<CollisionBox>()
            .register_component::<Bullet>();

        let player_id = world.create_entity();
        world.add_component_to_entity(player_id, Player::new(Some("player".to_string()), None));
        world.add_component_to_entity(
            player_id,
            Transform {
                position: ((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT - 32) as f32),
                velocity: (0.0, 0.0),
                direction: (0.0, 1.0),
            },
        );
        world.add_component_to_entity(
            player_id,
            CollisionBox {
                width: 32,
                height: 32,
            },
        );


        world
            .add_system(RenderSystem::new(&window))
            .add_system(EntitySystem::new())
            .add_system(BulletSystem::new())
            .add_system(PlayerSystem {})
            .add_system(CollisionSystem {})
            .add_system(GuiSystem {});

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

                    if let Some(world) = &mut self.world {
                         world.get_resource_mut::<Input>()
                        .map(|input| {  
                            match key {
                                KeyCode::KeyA => input.left_pressed = pressed,
                                KeyCode::KeyD => input.right_pressed = pressed,
                                KeyCode::KeyW => input.up_pressed = pressed,
                                KeyCode::KeyS => input.down_pressed = pressed,
                                KeyCode::Space => input.shoot_pressed = pressed,
                                _ => {}
                            }
                        });
                    }

                    
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
                    world.get_resource_mut::<GameState>()
                        .map(|game_state| {
                            let now = Instant::now();
                            if let Some(last_time) = game_state.last_time {
                                game_state.delta_time = now.duration_since(last_time).as_millis() as f32;
                            }
                            game_state.last_time = Some(now);
                        });
                 

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
