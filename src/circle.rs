use iced::{widget::canvas::Path, Point, Vector};
pub struct Circle {
    pub radius: f32,
    pub position: Vector,
    pub direction: Vector,
    pub show_center: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 25.0,
            position: Vector::new(0.0, 0.0),
            direction: Vector::new(-1.0, 0.0),
            show_center: true,
        }
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Self {
            radius: self.radius,
            position: self.position.clone(),
            direction: self.direction.clone(),
            show_center: self.show_center,
        }
    }
}

impl Circle {
    // Function for setting the radius of a circle
    pub fn set_radius(&self, radius: f32) -> Self {
        let mut circle = self.clone();
        circle.radius = radius;
        circle
    }

    // Function for setting the position, if x or y is None it doesn't change them
    pub fn set_position(&self, (x_position, y_position): (Option<f32>, Option<f32>)) -> Self {
        let mut circle: Circle = self.clone();
        circle.position.x = match x_position {
            Some(x) => x,
            None => self.position.x,
        };
        circle.position.y = match y_position {
            Some(y) => y,
            None => self.position.y,
        };
        circle
    }

    // Set the position of the circle at {distance} from {target}
    pub fn bound_to_target(&mut self, target: Vector, distance: f32) {
        self.position = target + self.direction * distance;
    }

    // Point the direction vector towards a new target
    pub fn set_target(&mut self, target: Vector) {
        self.direction = target - self.position;
    }

    // Normalize the direction vector
    pub fn normalize_direction(&mut self) {
        self.direction = self.direction
            * (1.0 / (self.direction.x.powf(2.0) + self.direction.y.powf(2.0)).sqrt());
    }

    // Function returning a path of the circle
    pub fn path(&self, frame_center: Point) -> Path {
        Path::circle(frame_center + self.position, self.radius)
    }

    // Function used for displaying the center marker
    pub fn center_path(&self, frame_center: Point) -> Path {
        Path::circle(frame_center + self.position, self.radius / 10.0)
    }
}
