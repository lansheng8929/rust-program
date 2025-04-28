use ecs_rust::component::Component;

pub struct Player {
    pub name: &'static str,
    pub position: (u32, u32),
    pub velocity: (i32, i32),
}

impl Component for Player {}
