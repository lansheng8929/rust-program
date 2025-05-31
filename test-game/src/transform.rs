use my_ecs_rust::component::Component;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub direction: (f32, f32),
}

impl Transform {}

impl Component for Transform {}
