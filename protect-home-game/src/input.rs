#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub shoot_pressed: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            shoot_pressed: false,
        }
    }
}

impl Input {}
