

pub struct EventWriter<'w, E: BufferedEvent> {
    #[system_param(validation_message = "BufferedEvent not initialized")]
    events: ResMut<'w, Events<E>>,
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
