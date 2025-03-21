#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_overlapping(&self, other: &Rectangle) -> bool {
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

    pub fn contains_point(&self, point_x: i32, point_y: i32) -> bool {
        point_x >= self.x
            && point_x < self.x + self.width as i32
            && point_y >= self.y
            && point_y < self.y + self.height as i32
    }
}
