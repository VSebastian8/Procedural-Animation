use iced::{widget::canvas::Path, Point, Vector};
pub struct Circle {
    pub position: Vector,
    pub radius: f32,
    pub show_center: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0].into(),
            radius: 25.0,
            show_center: true,
        }
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            radius: self.radius,
            show_center: self.show_center,
        }
    }
}

impl Circle {
    pub fn randomize_position(&self) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut randomized_circle = self.clone();
        randomized_circle.position =
            [rng.gen_range(-400.0..400.0), rng.gen_range(-300.0..300.0)].into();
        randomized_circle
    }

    pub fn path(&self, frame_center: Point) -> Path {
        Path::circle(frame_center + self.position, self.radius)
    }
    pub fn center_path(&self, frame_center: Point) -> Path {
        Path::circle(frame_center + self.position, self.radius / 10.0)
    }
}
