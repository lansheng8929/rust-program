use crate::app::App;

pub trait Plugin {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    
    fn build(&self, app: &mut App);
    
    fn is_unique(&self) -> bool {
        true
    }
}


/// 插件状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginsState {
    Adding,
    Ready,
    Cleaned,
    Finished,
}

/// 插件组 trait
pub trait Plugins<M> {
    fn add_to_app(self, app: &mut App);
}

impl<P: Plugin + 'static> Plugins<()> for P {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self);
    }
}

impl<P: Plugin + 'static> Plugins<()> for Vec<P> {
    fn add_to_app(self, app: &mut App) {
        for plugin in self {
            app.add_plugin(plugin);
        }
    }
}
