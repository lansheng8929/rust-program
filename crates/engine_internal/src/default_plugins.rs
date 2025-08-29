use engine_app::plugin_group;

plugin_group! {
    pub struct MinimalPlugins {
        engine_winit::WinitPlugin,
    }
}

mod test {
    use engine_app::prelude::*;

    #[test]
    fn  test_plugin_group_creation() {
        let builder = super::MinimalPlugins.build();
        assert_eq!(builder.len(), 1);
        assert!(builder.contains::<engine_winit::WinitPlugin>());
    }
}