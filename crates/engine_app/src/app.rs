use engine_ecs::prelude::*;

use crate::plugin::*;

/// 应用程序退出类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppExit {
    Success,
    Error(u8),
}

impl Default for AppExit {
    fn default() -> Self {
        Self::Success
    }
}

/// 简化的应用程序结构
pub struct App {
    world: World,
    plugins: Vec<Box<dyn Plugin>>,
    runner: Option<Box<dyn FnOnce(App) -> AppExit>>,
    plugins_state: PluginsState,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App {{ plugins: {}, state: {:?} }}", self.plugins.len(), self.plugins_state)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// 创建新的应用程序
    pub fn new() -> App {
        App {
            world: World::new(),
            plugins: Vec::new(),
            runner: None,
            plugins_state: PluginsState::Adding,
        }
    }
    
    /// 添加插件
    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }
    
    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        if matches!(
            self.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }
        plugins.add_to_app(self);
        self
    }

    pub fn add_boxed_plugin(
        &mut self,
        plugin: Box<dyn Plugin>,
    ) -> &mut Self {
        // 检查插件状态，确保不能在清理或完成后添加插件
        if matches!(
            self.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }
        
        // 检查插件唯一性
        if plugin.is_unique() {
            let plugin_name = plugin.name();
            for existing_plugin in &self.plugins {
                if existing_plugin.name() == plugin_name {
                    panic!("Plugin '{}' was added multiple times but is unique", plugin_name);
                }
            }
        }
        
        // 添加插件到列表
        self.plugins.push(plugin);
        self
    }
    
    /// 运行应用程序
    pub fn run(&mut self) -> AppExit {
        // 构建所有插件
        let plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins {
            plugin.build(self);
        }
        
        self.plugins_state = PluginsState::Ready;
        
        // 运行主循环
        if let Some(runner) = self.runner.take() {
            let app = std::mem::replace(self, App::new());
            runner(app)
        } else {
            self.run_once()
        }
    }

    /// 设置一个持续运行的 runner
    pub fn set_loop_runner(&mut self) -> &mut Self {
        self.set_runner(|mut app| {
            loop {
                let exit = app.run_once();
                if exit != AppExit::Success {
                    return exit;
                }
                // 可以在这里添加帧率控制或事件处理
            }
        })
    }
    
    /// 运行一次更新
    pub fn run_once(&mut self) -> AppExit {
        // 更新 ECS 世界（运行所有系统）
        self.world.update();
        AppExit::Success
    }
    
    /// 设置自定义运行器
    pub fn set_runner<F>(&mut self, runner: F) -> &mut Self 
    where
        F: FnOnce(App) -> AppExit + 'static,
    {
        self.runner = Some(Box::new(runner));
        self
    }
    
    /// 获取世界的可变引用
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    /// 获取插件状态
    pub fn plugins_state(&self) -> PluginsState {
        self.plugins_state.clone()
    }
    
    /// 清理应用程序
    pub fn cleanup(&mut self) {
        self.plugins_state = PluginsState::Cleaned;
    }
    
    /// 完成应用程序
    pub fn finish(&mut self) {
        self.plugins_state = PluginsState::Finished;
    }
}


// 示例系统结构体
pub struct HelloWorldSystem;

impl System for HelloWorldSystem {
    fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        println!("Hello, World!");
    }
}

// 示例插件
pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut().add_system(HelloWorldSystem);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.plugins.len(), 0);
    }

    #[test]
    fn test_plugin_addition() {
        let mut app = App::new();
        app.add_plugin(HelloWorldPlugin);
        assert_eq!(app.plugins.len(), 1);
    }

    #[test]
    fn test_system_addition() {
        struct TestSystem;
        
        impl System for TestSystem {
            fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
                println!("Test system");
            }
        }
        
        let mut app = App::new();
        app.world_mut().add_system(TestSystem);
        
        // 测试系统添加成功（没有直接验证方式，但不应该崩溃）
        app.run_once();
    }

    #[test]
    fn test_set_runner_error() {
        struct ExitSystem;
        impl System for ExitSystem {
            fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
                println!("Exit system triggered");
            }
        }

        struct ErrorPlugin;
        impl Plugin for ErrorPlugin {
            fn build(&self, app: &mut App) {
                app.world_mut().add_system(ExitSystem);
            }
        }

        let mut app = App::new();
        app.add_plugin(ErrorPlugin);
        app.set_runner(|mut app| {
            let mut count = 0;
            loop {
                let exit = app.run_once();
                count += 1;
                if count >= 3 {
                    return AppExit::Error(42);
                }
                if exit != AppExit::Success {
                    return exit;
                }
            }
        });

        let result = app.run();
        assert_eq!(result, AppExit::Error(42));
    }

    #[test]
    fn test_set_runner_success() {
        let mut app = App::new();
        use std::rc::Rc;
        use std::cell::RefCell;

        let called = Rc::new(RefCell::new(false));
        struct DummySystem {
            count: u32,
            called: Rc<RefCell<bool>>,
        }
        impl System for DummySystem {
            fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
                self.count += 1;
                *self.called.borrow_mut() = true;
                println!("Dummy system called {} times", self.count);
            }
        }
        app.world_mut().add_system(DummySystem {count: 0, called: Rc::clone(&called) });
        // 设置循环运行器，但run_once总是返回Success，会无限循环，这里人为终止
        app.set_runner(|mut app| {
            let mut count = 0;
            loop {
                let exit = app.run_once();
                count += 1;
                if count >= 50 {
                    return exit;
                }
                if exit != AppExit::Success {
                    return exit;
                }
            }
        });
        let result = app.run();
        assert_eq!(result, AppExit::Success);
        assert!(*called.borrow());
    }

    #[test]
    fn text_set_loop_runner() {

        let mut app = App::new();
        use std::rc::Rc;
        use std::cell::RefCell;

        let called = Rc::new(RefCell::new(false));
        struct DummySystem {
            count: u32,
            called: Rc<RefCell<bool>>,
        }
        impl System for DummySystem {
            fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
                self.count += 1;
                *self.called.borrow_mut() = true;
                println!("Dummy system called {} times", self.count);
            }
        }
        app.world_mut().add_system(DummySystem {count: 0, called: Rc::clone(&called) });
        // 设置循环运行器，但run_once总是返回Success，会无限循环，这里人为终止
        app.set_loop_runner();
        let result = app.run();
        assert_eq!(result, AppExit::Success);
        assert!(*called.borrow());
    }

    #[test]
    fn test_plugins_state_management() {
        let mut app = App::new();
        assert_eq!(app.plugins_state(), PluginsState::Adding);
        
        app.cleanup();
        assert_eq!(app.plugins_state(), PluginsState::Cleaned);
    }

    #[test]
    #[should_panic(expected = "Plugins cannot be added after App::cleanup() or App::finish() has been called.")]
    fn test_add_plugins_after_cleanup() {
        let mut app = App::new();
        app.cleanup();
        app.add_plugins::<()>(HelloWorldPlugin);
    }

    #[test]
    #[should_panic(expected = "Plugins cannot be added after App::cleanup() or App::finish() has been called.")]
    fn test_add_plugins_after_finish() {
        let mut app = App::new();
        app.finish();
        app.add_plugins::<()>(HelloWorldPlugin);
    }

    #[test]
    fn test_add_plugins_with_vec() {
        let mut app = App::new();
        let plugins = vec![HelloWorldPlugin, HelloWorldPlugin];
        app.add_plugins(plugins);
        assert_eq!(app.plugins.len(), 2);
    }
}
