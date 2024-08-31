use iced::{widget::canvas::Path, Point, Transformation, Vector};
pub struct Circle {
    pub radius: f32,
    pub radius_scale: Transformation,
    pub position: Vector,
    pub direction: Vector,
    pub show_center: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 25.0,
            radius_scale: Transformation::scale(-25.0),
            position: [0.0, 0.0].into(),
            direction: [-1.0, 0.0].into(),
            show_center: true,
        }
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Self {
            radius: self.radius,
            radius_scale: Transformation::scale(-1.0 * self.radius),
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
        circle.radius_scale = Transformation::scale(-1.0 * radius);
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

    // Function for randomizing the circle's center position
    pub fn randomize_position(&self) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut randomized_circle = self.clone();
        randomized_circle.position =
            [rng.gen_range(-400.0..400.0), rng.gen_range(-300.0..300.0)].into();
        randomized_circle
    }

    // Set the position of the circle at {distance} from {target}
    pub fn bound_to_target(&mut self, target: Vector, distance: Transformation) {
        self.position = target + self.direction * distance;
    }

    // Point the direction vector towards a new target
    pub fn set_target(&mut self, target: Vector) {
        self.direction = target - self.position;
    }

    // Normalize the direction vector
    pub fn normalize_direction(&mut self) {
        self.direction = self.direction
            * Transformation::scale(
                1.0 / (self.direction.x.powf(2.0) + self.direction.y.powf(2.0)).sqrt(),
            );
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
