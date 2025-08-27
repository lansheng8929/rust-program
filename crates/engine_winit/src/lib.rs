use std::cell::RefCell;
use crate::winit_windows::WinitWindows;

mod winit_windows;

pub mod prelude {
    pub use super::winit_windows::*;
}

thread_local! {
    // 全局的 WinitWindows 实例，每个线程中都维护一个独立的静态变量
    pub static WINIT_WINDOWS: RefCell<WinitWindows> = RefCell::new(WinitWindows::new());
}