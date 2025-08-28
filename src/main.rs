use my_macro::CustomDebug;
use engine_internal::prelude::*;

// 自定义声明宏
macro_rules! my_println {
    // 匹配不带参数的调用（如 `my_println!()`）
    () => {
        println!()
    };
    // 匹配带参数的调用（如 `my_println!("Hello")`）
    ($msg:expr) => {
        println!("{}", $msg)
    };
}

#[derive(CustomDebug)]
struct Test {
    a: u32,
    b: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    
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

    app.run();

    Ok(())
}
