#![deny(clippy::all)]
#![forbid(unsafe_code)]
// #![windows_subsystem = "windows"]

use app::App;
use core::time::Duration;
use rust_embed::RustEmbed;
use winit::event_loop::{ControlFlow, EventLoop};

mod animation;
mod app;
mod bullet;
mod cursor_state;
mod enemy;
mod game_data;
mod gui;
mod input_state;
mod player;
mod rectangle;
mod sound;
mod spatial_grid;
mod uitils;
mod world;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

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
    let _ = event_loop.run_app(&mut app);
}
