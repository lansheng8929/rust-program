use std::collections::HashMap;
use engine_ecs::prelude::*;

/// 插件特征
pub trait Plugin {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    
    fn build(&self, app: &mut App);
    
    fn is_unique(&self) -> bool {
        true
    }
}

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

/// 简化的调度器
pub struct Schedule {
    systems: Vec<Box<dyn FnMut() + Send + Sync>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    
    pub fn add_system<F>(&mut self, system: F) 
    where 
        F: FnMut() + Send + Sync + 'static,
    {
        self.systems.push(Box::new(system));
    }
    
    pub fn run(&mut self) {
        for system in &mut self.systems {
            system();
        }
    }
}

/// 简化的世界
pub struct World {
    schedules: HashMap<String, Schedule>,
    resources: ResourceManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
            resources: ResourceManager::new(),
        }
    }
    
    pub fn add_schedule(&mut self, name: String, schedule: Schedule) {
        self.schedules.insert(name, schedule);
    }
    
    pub fn run_schedule(&mut self, name: &str) {
        if let Some(schedule) = self.schedules.get_mut(name) {
            schedule.run();
        }
    }
    
    pub fn get_schedule_mut(&mut self, name: &str) -> Option<&mut Schedule> {
        self.schedules.get_mut(name)
    }
    
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resources.add(resource);
    }
}

/// 简化的应用程序结构
pub struct App {
    world: World,
    plugins: Vec<Box<dyn Plugin>>,
    runner: Option<Box<dyn FnOnce(App) -> AppExit>>,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App {{ plugins: {} }}", self.plugins.len())
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
        }
    }
    
    /// 添加插件
    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }
    
    /// 添加系统到指定调度器
    pub fn add_system<F>(&mut self, schedule_name: &str, system: F) -> &mut Self 
    where 
        F: FnMut() + Send + Sync + 'static,
    {
        if let Some(schedule) = self.world.get_schedule_mut(schedule_name) {
            schedule.add_system(system);
        } else {
            let mut new_schedule = Schedule::new();
            new_schedule.add_system(system);
            self.world.add_schedule(schedule_name.to_string(), new_schedule);
        }
        self
    }
    
    /// 运行应用程序
    pub fn run(&mut self) -> AppExit {
        // 构建所有插件
        let plugins = std::mem::take(&mut self.plugins);
        for plugin in plugins {
            plugin.build(self);
        }
        
        // 运行主循环
        if let Some(runner) = self.runner.take() {
            let app = std::mem::replace(self, App::new());
            runner(app)
        } else {
            self.run_once()
        }
    }
    
    /// 运行一次更新
    pub fn run_once(&mut self) -> AppExit {
        // 运行主调度器
        self.world.run_schedule("main");
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
    
    /// 插入资源
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }


}

// 示例插件
pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system("main", || {
            println!("Hello, World!");
        });
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
        let mut app = App::new();
        app.add_system("test", || {
            println!("Test system");
        });
        
        // 验证调度器已创建
        assert!(app.world.schedules.contains_key("test"));
    }
}
