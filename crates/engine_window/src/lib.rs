extern crate alloc;

mod raw_handle;
mod window;
mod event;

pub mod prelude {
    pub use crate::{
        event::*,
        window::*,
        raw_handle::*,
    };
}
