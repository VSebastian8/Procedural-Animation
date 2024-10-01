use iced::{widget::canvas::Path, Point, Vector};
pub struct Circle {
    pub radius: f32,
    pub offset: f32,
    pub position: Vector,
    pub direction: Vector,
    pub show_center: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 25.0,
            offset: 0.0,
            position: Vector::new(0.0, 0.0),
            direction: Vector::new(-1.0, 0.0),
            show_center: false,
        }
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        Self {
            radius: self.radius,
            offset: self.offset,
            position: self.position.clone(),
            direction: self.direction.clone(),
            show_center: self.show_center,
        }
    }
}

#[allow(dead_code)]
impl Circle {
    // Function for setting the radius of a circle
    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    // Function for setting the offset of the center from the frontier of the previous circle
    pub fn set_offset(&mut self, offset: f32) -> &mut Self {
        self.offset = offset;
        self
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
        self.position = target + self.direction * (distance + self.offset) * (-1.0);
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

    // Function to get the point on the circle outline corresponding to a direction
    pub fn point_on_circle(&self, dir: Vector) -> Vector {
        self.position + dir * self.radius
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
