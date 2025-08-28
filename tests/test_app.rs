use std::marker::PhantomData;

use engine_internal::prelude::*;

pub struct HelloWorldSystem;

impl System for HelloWorldSystem {
    fn update(&mut self, _manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        println!("Hello, World!");
    }
}

pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App) {
        app.set_loop_runner();
        app.world_mut().add_system(HelloWorldSystem);
    }
}


fn create_test_app() -> App {
    let mut app = App::new();

    app.add_plugin(HelloWorldPlugin);
    let mut winit_plugin = WinitPlugin::<WakeUp>::default();
    winit_plugin.run_on_any_thread = true;
    app.add_plugin(winit_plugin);

    // 注册组件
    app.world_mut().register_component::<Window>(); 
    // 创建实体
    let window_id = app.world_mut().create_entity();
    // 创建组件
    let window = Window {
        title: "This is window 0!".to_string(),
        ..Default::default()
    };
    // 添加组件到实体
    app.world_mut().add_component_to_entity(window_id, window);

    app
}

#[test]
fn test_window_title() {
    let mut app = create_test_app();

    app.run();

    let window: Vec<&Window> = app.world_mut().query::<Window>();
    let window: &Window = *window.first().unwrap();

    assert_eq!(window.title, "This is window 0!");
}