use std::{cell::RefCell, marker::PhantomData, time::Instant};
use engine_app::prelude::*;
use engine_ecs::prelude::*;
use engine_window::prelude::*;
use winit::{application::ApplicationHandler, event::{StartCause, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy}, window::{WindowId, Window}};

use crate::{winit_windows::WinitWindows};

mod winit_windows;
mod system;


#[cfg(target_os = "macos")]
mod macos;

thread_local! {
    // 全局的 WinitWindows 实例，每个线程中都维护一个独立的静态变量
    pub static WINIT_WINDOWS: RefCell<WinitWindows> = RefCell::new(WinitWindows::new());
}

// 包装 EventLoopProxy 以便作为资源存储在 World 中
pub struct EventLoopProxyWrapper<T: 'static>(EventLoopProxy<T>);
impl<T: 'static> Resource for EventLoopProxyWrapper<T> {
}

// 标记事件类型的特征，确保事件可以在线程间传递
#[derive(Debug, Default, Clone, Copy, BufferedEvent)]
pub struct WakeUp;

/// 标记组件，用于跟踪窗口是否已经在 Winit 中创建
#[derive(Debug, Default, Clone, Copy)]
pub struct WinitWindowCreated;

impl Component for WinitWindowCreated {}


// Winit 应用程序运行状态
pub struct WinitAppRunnerState<T: BufferedEvent> {
    app: App,
    app_exit: Option<AppExit>,
    window: Option<Window>,
    marker: PhantomData<T>,
    wait_elapsed: bool,
    redraw_requested: bool,
    lifecycle: AppLifecycle,
}
impl<T: BufferedEvent> WinitAppRunnerState<T> {
    fn new(app: App) -> Self {
        Self {
            app,
            app_exit: None,
            window: None,
            marker: PhantomData,
            wait_elapsed: true,
            redraw_requested: false,
            lifecycle: AppLifecycle::Idle,
        }
    }
}

impl<T: BufferedEvent> ApplicationHandler<T> for WinitAppRunnerState<T> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if event_loop.exiting() {
            return;
        }

        self.wait_elapsed = match cause {
            StartCause::WaitCancelled {
                requested_resume: Some(resume),
                ..
            } => resume <= Instant::now(),
            _ => true,
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        println!("window event: {:?}", event);

        WINIT_WINDOWS.with_borrow(|winit_windows| {
            let Some(window) = winit_windows.get_window_entity(window_id) else {
                println!("无法找到与窗口 ID {:?} 关联的实体", window_id);
                return;
            };
            println!("{:?}", window);
        });

        match event {
            WindowEvent::CloseRequested => {
                println!("Test window close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // 标记需要重绘
                self.redraw_requested = true;
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        self.create_pending_windows(event_loop);

        // 应用恢复时请求重绘
        self.redraw_requested = true;
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        println!("about_to_wait called, wait_elapsed: {}", self.wait_elapsed);

        self.create_pending_windows(event_loop);

        #[cfg(not(target_os = "windows"))]
        self.redraw_requested(event_loop);
    }
}

impl<T: BufferedEvent> WinitAppRunnerState<T> {
    fn create_pending_windows(&mut self, event_loop: &ActiveEventLoop) {

        let world = self.app.world_mut();

        // 第一步：收集所有拥有 Window 组件的实体 ID 和窗口数据
        let all_window_entities = world.query_with_entities::<engine_window::prelude::Window>();
        let mut entity_window_pairs: Vec<(usize, engine_window::prelude::Window)> = Vec::new();

        for (entity_id, window) in all_window_entities {
            entity_window_pairs.push((entity_id, window.clone()));
        }

        // 第二步：过滤出需要创建的窗口（排除已经创建的）
        let mut windows_to_create: Vec<(usize, engine_window::prelude::Window)> = Vec::new();

        for (entity_id, window) in entity_window_pairs {
            // 检查是否已经创建了 winit 窗口
            if !world.has_component::<WinitWindowCreated>(entity_id) {
                windows_to_create.push((entity_id, window));
            }
        }


        // 创建窗口
        WINIT_WINDOWS.with_borrow_mut(|winit_windows| {
            for (entity_id, window) in &windows_to_create {
                let entity = Entity::from_raw(*entity_id);

                println!("Creating window '{}' for entity {}", window.title, entity_id);

                winit_windows.create_window(event_loop, entity, window);
            }
        });

        // 第四步：标记已创建的窗口
        for (entity_id, _) in windows_to_create {
            world.add_component_to_entity(entity_id, WinitWindowCreated);
        }

    }

    fn redraw_requested(&mut self, event_loop: &ActiveEventLoop) {
        if self.redraw_requested && self.lifecycle != AppLifecycle::Suspended {
            WINIT_WINDOWS.with_borrow(|winit_windows| {
                for window in winit_windows.windows.values() {
                    window.request_redraw();
                }
            });
            self.redraw_requested = false;
        }

        if let Some(app_exit) = self.app.should_exit() {
            self.app_exit = Some(app_exit);

            event_loop.exit();
        }
    }
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

            // 自动注册 WinitWindowCreated 组件
            app.world_mut().register_component::<WinitWindowCreated>();

            let mut event_loop_builder = EventLoop::<T>::with_user_event();

            #[cfg(all(target_os = "linux", feature = "x11"))]
            {
                println!("启用 X11 支持: {}", self.run_on_any_thread);
                #[allow(unused_imports)]
                use winit::platform::x11::EventLoopBuilderExtX11;
                event_loop_builder.with_any_thread(self.run_on_any_thread);
            }

            #[cfg(target_os = "macos")]
            {
                println!("启用 macOS 支持: {}", self.run_on_any_thread);
                use winit::platform::macos::ActivationPolicy;
                #[allow(unused_imports)]
                use winit::platform::macos::EventLoopBuilderExtMacOS;

                // 设置激活策略为 Regular（标准桌面应用）
                event_loop_builder.with_activation_policy(ActivationPolicy::Regular);

                // 启用默认菜单栏
                event_loop_builder.with_default_menu(true);
            }

            let event_loop = event_loop_builder.build().expect("事件循环创建失败");

            app.set_runner(|app| winit_runner(app, event_loop));
        }
}

pub mod prelude {
    pub use super::winit_windows::*;
    pub use super::WinitPlugin;
    pub use super::WakeUp;

}
