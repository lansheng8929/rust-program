use engine_ecs::prelude::*;

#[derive(BufferedEvent, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLifecycle {
    /// The application is not started yet.
    Idle,
    /// The application is running.
    Running,
    /// The application is going to be suspended.
    /// Applications have one frame to react to this event before being paused in the background.
    WillSuspend,
    /// The application was suspended.
    Suspended,
    /// The application is going to be resumed.
    /// Applications have one extra frame to react to this event before being fully resumed.
    WillResume,
}

pub struct EventWriter<E: BufferedEvent> {
    events: Events<E>,
}

pub struct WindowResized {
    /// Window that has changed.
    pub window: Entity,
    /// The new logical width of the window.
    pub width: f32,
    /// The new logical height of the window.
    pub height: f32,
}


pub struct WindowBackendScaleFactorChanged {
    /// Window that had its scale factor changed by the backend.
    pub window: Entity,
    /// The new scale factor.
    pub scale_factor: f64,
}


pub struct WindowScaleFactorChanged {
    /// Window that had its scale factor changed.
    pub window: Entity,
    /// The new scale factor.
    pub scale_factor: f64,
}
