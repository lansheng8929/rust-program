use engine_winit as winit;
use engine_window as window;
use engine_app as app;
use engine_math as math;
use engine_platform as platform;
use engine_ecs as ecs;

mod default_plugins;

pub mod prelude {
    pub use super::app::prelude::*;
    pub use super::math::prelude::*;
    pub use super::platform::prelude::*;
    pub use super::window::prelude::*;
    pub use super::winit::prelude::*;
    pub use super::ecs::prelude::*;

    pub use super::default_plugins::*;
}