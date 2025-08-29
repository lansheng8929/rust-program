use std::any::TypeId;
use std::collections::HashMap;

use crate::app::App;
use crate::plugin::Plugin;


/// 简化的插件组宏
#[macro_export]
macro_rules! plugin_group {
    {
        $(#[$meta:meta])*
        $vis:vis struct $group:ident {
            $(
                $plugin:ty
            ),* $(,)?
        }
    } => {
        $(#[$meta])*
        $vis struct $group;

        impl $crate::prelude::PluginGroup for $group {
            fn build(self) -> $crate::prelude::PluginGroupBuilder {
                let mut group = $crate::prelude::PluginGroupBuilder::start::<Self>();
                
                $(
                    group = group.add(<$plugin>::default());
                )*
                
                group
            }
        }
    };
}


/// 简化的插件条目
struct PluginEntry {
    plugin: Box<dyn Plugin>,
    enabled: bool,
}

/// 简化的插件组 trait
pub trait PluginGroup: Sized {
    /// 构建插件组，返回插件构建器
    fn build(self) -> PluginGroupBuilder;
    
    /// 获取插件组名称，主要用于调试
    fn name() -> String {
        std::any::type_name::<Self>().to_string()
    }
}

/// 插件组构建器
pub struct PluginGroupBuilder {
    group_name: String,
    plugins: HashMap<TypeId, PluginEntry>,
    order: Vec<TypeId>,
}

impl PluginGroupBuilder {
    /// 创建新的插件组构建器
    pub fn start<PG: PluginGroup>() -> Self {
        Self {
            group_name: PG::name(),
            plugins: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// 检查是否包含指定插件
    pub fn contains<T: Plugin + 'static>(&self) -> bool {
        self.plugins.contains_key(&TypeId::of::<T>())
    }

    /// 检查指定插件是否启用
    pub fn enabled<T: Plugin + 'static>(&self) -> bool {
        self.plugins
            .get(&TypeId::of::<T>())
            .map(|entry| entry.enabled)
            .unwrap_or(false)
    }

    /// 添加插件到组末尾
    pub fn add<T: Plugin + 'static>(mut self, plugin: T) -> Self {
        let type_id = TypeId::of::<T>();
        
        // 如果插件已存在，先移除旧的顺序
        if self.plugins.contains_key(&type_id) {
            self.order.retain(|&id| id != type_id);
        }
        
        // 添加新插件
        self.order.push(type_id);
        self.plugins.insert(type_id, PluginEntry {
            plugin: Box::new(plugin),
            enabled: true,
        });
        
        self
    }

    /// 添加插件组
    pub fn add_group(mut self, group: impl PluginGroup) -> Self {
        let other_builder = group.build();
        
        // 合并插件
        for type_id in other_builder.order {
            if let Some(entry) = other_builder.plugins.get(&type_id) {
                // 如果插件已存在，先移除旧的顺序
                if self.plugins.contains_key(&type_id) {
                    self.order.retain(|&id| id != type_id);
                }
                
                self.order.push(type_id);
                // 注意：这里我们不能直接移动 Box<dyn Plugin>，所以跳过已存在的插件
                if !self.plugins.contains_key(&type_id) {
                    println!("Warning: Cannot move plugin from another group, skipping: {:?}", type_id);
                }
            }
        }
        
        self
    }

    /// 启用插件
    pub fn enable<T: Plugin + 'static>(mut self) -> Self {
        if let Some(entry) = self.plugins.get_mut(&TypeId::of::<T>()) {
            entry.enabled = true;
        }
        self
    }

    /// 禁用插件
    pub fn disable<T: Plugin + 'static>(mut self) -> Self {
        if let Some(entry) = self.plugins.get_mut(&TypeId::of::<T>()) {
            entry.enabled = false;
        }
        self
    }

    /// 完成构建，将所有启用的插件添加到应用
    pub fn finish(mut self, app: &mut App) {
        for type_id in self.order {
            if let Some(entry) = self.plugins.remove(&type_id) {
                if entry.enabled {
                    println!("Adding plugin: {}", entry.plugin.name());
                    app.add_plugins(entry.plugin);
                }
            }
        }
    }

    /// 获取插件数量
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// 获取组名
    pub fn group_name(&self) -> &str {
        &self.group_name
    }
}

impl PluginGroup for PluginGroupBuilder {
    fn build(self) -> PluginGroupBuilder {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::Plugin;
    use crate::app::App;

    #[derive(Default)]
    struct TestPlugin1;
    
    impl Plugin for TestPlugin1 {
        fn build(&self, _app: &mut App) {
            println!("TestPlugin1 build");
        }
    }

    #[derive(Default)]
    struct TestPlugin2;
    
    impl Plugin for TestPlugin2 {
        fn build(&self, _app: &mut App) {
            println!("TestPlugin2 build");
        }
    }

    plugin_group! {
        /// 测试插件组
        pub struct TestPluginGroup {
            TestPlugin1,
            TestPlugin2,
        }
    }

    #[test]
    fn test_plugin_group_creation() {
        let builder = TestPluginGroup.build();
        assert_eq!(builder.len(), 2);
        assert!(builder.contains::<TestPlugin1>());
        assert!(builder.contains::<TestPlugin2>());
    }

    #[test]
    fn test_plugin_enable_disable() {
        let builder = TestPluginGroup.build()
            .disable::<TestPlugin1>()
            .enable::<TestPlugin2>();
        
        assert!(!builder.enabled::<TestPlugin1>());
        assert!(builder.enabled::<TestPlugin2>());
    }

    #[test]
    fn test_plugin_group_finish() {
        let mut app = App::new();
        let builder = TestPluginGroup.build();
        
        // 这应该会打印插件构建信息
        builder.finish(&mut app);
    }
}
