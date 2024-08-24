use iced::{widget::canvas::Path, Color, Point, Vector};
pub struct Circle {
    pub radius: f32,
    pub position: Vector,
    pub color: Color,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0].into(),
            radius: 75.0,
            color: Color::from_rgba8(0, 179, 134, 0.8),
        }
    }
}

impl Circle {
    pub fn path(&self, frame_center: Point) -> Path {
        Path::circle(frame_center + self.position, self.radius)
    }
}
