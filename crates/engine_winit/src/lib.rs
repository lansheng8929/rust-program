use std::{cell::RefCell, marker::PhantomData};
use engine_app::prelude::*;
use engine_ecs::prelude::*;
use winit::{application::ApplicationHandler, event::{StartCause, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy}, window::{WindowId, Window}};

use crate::winit_windows::WinitWindows;

mod winit_windows;


thread_local! {
    // 全局的 WinitWindows 实例，每个线程中都维护一个独立的静态变量
    pub static WINIT_WINDOWS: RefCell<WinitWindows> = RefCell::new(WinitWindows::new());
}

// 包装 EventLoopProxy 以便作为资源存储在 World 中
pub struct EventLoopProxyWrapper<T: 'static>(EventLoopProxy<T>);
impl<T: 'static> Resource for EventLoopProxyWrapper<T> {
}

// 标记事件类型的特征，确保事件可以在线程间传递
#[derive(Debug, Default, Clone, Copy,BufferedEvent)]
pub struct WakeUp;

// Winit 应用程序运行状态
pub struct WinitAppRunnerState<T: BufferedEvent> {
    app: App,
    window: Option<Window>,
    marker: PhantomData<T>,
}
impl<T: BufferedEvent> WinitAppRunnerState<T> {
    fn new(mut app: App) -> Self {
        Self {
            app,
            window: None,
            marker: PhantomData,
        }
    }
}
impl<T: BufferedEvent> ApplicationHandler<T> for WinitAppRunnerState<T> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        println!("新事件: {:?}", cause);

        if let StartCause::Init = cause {
            let window_attributes = Window::default_attributes()
                .with_title("Test Winit Window")
                .with_inner_size(winit::dpi::LogicalSize::new(500.0, 500.0));
            
            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    println!("Test window created successfully!");
                    self.window = Some(window);
                }
                Err(e) => {
                    eprintln!("Failed to create test window: {:?}", e);
                    event_loop.exit();
                }
            }
        }       
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
            match event {
            WindowEvent::CloseRequested => {
                println!("Test window close requested");
                event_loop.exit();
            }
            _ => {}
        }
    }
        
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}
}

// Winit 运行器函数
pub fn winit_runner<T: BufferedEvent>(mut app: App, event_loop: EventLoop<T>) -> AppExit {
    // 检查插件状态，如果准备好则执行收尾和清理
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    // 将事件循环代理插入资源
    app.world_mut()
        .add_resource(EventLoopProxyWrapper(event_loop.create_proxy()));

    // 创建运行状态
    let runner_state = WinitAppRunnerState::new(app);

    // wasm 平台使用 spawn_app，其他平台使用 run_app
    let mut runner_state = runner_state;
    if let Err(err) = event_loop.run_app(&mut runner_state) {
        eprintln!("事件循环运行失败: {:?}", err);
        AppExit::Error(1)
    } else {
        AppExit::Success
    }
}


// Winit 插件结构体
#[derive(Default)]
pub struct WinitPlugin<T: BufferedEvent = WakeUp> {
    /// 是否允许在任意线程创建窗口和事件循环
    pub run_on_any_thread: bool,
    marker: PhantomData<T>,
}

impl<T: BufferedEvent> Plugin for WinitPlugin<T> {
    fn name(&self) -> &str {
        "bevy_winit::WinitPlugin"
    }

    fn build(&self, app: &mut App) {
        let mut event_loop_builder = EventLoop::<T>::with_user_event();

        #[cfg(all(target_os = "linux", feature = "x11"))]
        {
            println!("启用 X11 支持 {:}", self.run_on_any_thread);
            use winit::platform::x11::EventLoopBuilderExtX11;
            event_loop_builder.with_any_thread(self.run_on_any_thread);
        }

        let event_loop = event_loop_builder.build().expect("事件循环创建失败");

        app.set_runner(|app| winit_runner(app, event_loop));

    }
}

pub mod prelude {
    pub use super::winit_windows::*;
    pub use super::WinitPlugin;
    pub use super::BufferedEvent;
    pub use super::WakeUp;

}