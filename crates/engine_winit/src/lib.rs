

thread_local! {
    // 全局的 WinitWindows 实例，每个线程中都维护一个独立的静态变量
    pub static WINIT_WINDOWS: RefCell<WinitWindows> = const { RefCell::new(WinitWindows::new()) };
}