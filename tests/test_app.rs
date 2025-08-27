use engine_internal::prelude::*;



fn create_test_app() -> App {
    let mut app = App::new();

    // Note the use of `MinimalPlugins` instead of `DefaultPlugins`, as described above.
    // app.add_plugin(MinimalPlugins);
    // Inserting a `KeyCode` input resource allows us to inject keyboard inputs, as if the user had
    // pressed them.
    app.insert_resource(ButtonInput::<KeyCode>::default());

    // Spawning a fake window allows testing systems that require a window.
    app.world_mut().spawn(Window::default());

    app
}

#[test]
fn test_window_title() {
    let mut app = create_test_app();
    app.add_plugins(game_plugin);

    app.update();

    let window = app
        .world_mut()
        .query::<&Window>()
        .single(app.world())
        .unwrap();
    assert_eq!(window.title, "This is window 0!");
}