use std::{cell::RefCell, collections::HashMap};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    error::ExternalError,
    event_loop::ActiveEventLoop,
    monitor::{MonitorHandle, VideoModeHandle},
    window::{CursorGrabMode as WinitCursorGrabMode, Fullscreen, Window as WinitWindow, WindowId},
};


#[derive(Debug, Default)]
pub struct WinitWindows {
    /// Stores [`winit`] windows by window identifier.
    pub windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
    /// Maps entities to `winit` window identifiers.
    pub entity_to_winit: EntityHashMap<WindowId>,
    /// Maps `winit` window identifiers to entities.
    pub winit_to_entity: HashMap<WindowId, Entity>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}


thread_local! {
    // 全局的 WinitWindows 实例，每个线程中都维护一个独立的静态变量
    pub static WINIT_WINDOWS: RefCell<WinitWindows> = const { RefCell::new(WinitWindows::new()) };
}