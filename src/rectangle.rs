#[derive(Debug, Clone, Copy)]
pub(crate) struct Rectangle {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Rectangle {
    pub(crate) fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub(crate) fn is_overlapping(&self, other: &Rectangle) -> bool {
        // 计算每个矩形的边界
        let self_right = self.x + self.width as i32;
        let self_top = self.y + self.height as i32;
        let other_right = other.x + other.width as i32;
        let other_top = other.y + other.height as i32;

        !(self_right <= other.x
            || self.x >= other_right
            || self_top <= other.y
            || self.y >= other_top)
    }

    pub(crate) fn contains_point(&self, point_x: i32, point_y: i32) -> bool {
        point_x >= self.x
            && point_x < self.x + self.width as i32
            && point_y >= self.y
            && point_y < self.y + self.height as i32
    }
}
