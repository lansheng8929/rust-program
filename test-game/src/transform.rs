use ecs_rust::component::Component;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

impl Transform {}

impl Component for Transform {}
