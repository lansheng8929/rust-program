use ecs_rust::component::Component;

#[derive(Debug)]
pub struct Transform {
    pub position: (u32, u32),
    pub velocity: (i32, i32),
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: (0, 0),
            velocity: (0, 0),
        }
    }
}

impl Component for Transform {}
