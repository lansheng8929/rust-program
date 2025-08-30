use crate::{app::App, prelude::PluginGroup};

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

pub struct PluginMarker;
pub struct PluginGroupMarker;

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

impl Plugins<()> for Box<dyn Plugin> {
    fn add_to_app(self, app: &mut App) {
        app.add_boxed_plugin(self);
    }
}

impl<P: Plugin + 'static> Plugins<PluginMarker> for P {
    fn add_to_app(self, app: &mut App) {
        app.add_boxed_plugin(Box::new(self));
    }
}

 impl<P: PluginGroup> Plugins<PluginGroupMarker> for P {
    fn add_to_app(self, app: &mut App) {
            self.build().finish(app);
    }
}   