use crate::circle::*;
use iced::Transformation;

pub struct Chain {
    pub circles: Vec<Circle>,
    pub speed: Transformation,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            circles: vec![Circle::default()],
            speed: Transformation::scale(4.0),
        }
    }
}

impl Chain {
    // Function that sets the radius for each circle
    pub fn set_circles_radii(self, radius_array: Vec<f32>) -> Self {
        Self {
            circles: radius_array
                .into_iter()
                .map(|r| Circle::default().set_radius(r))
                .collect(),
            speed: self.speed,
        }
    }

    // Function that randomizez the position of each circle
    pub fn randomize_circles_positions(self) -> Self {
        Self {
            circles: self
                .circles
                .into_iter()
                .map(|c| c.randomize_position())
                .collect(),
            speed: self.speed,
        }
    }

    // Set the circle positions according to a function given as argument
    // The function takes a circle's index and radius and returns an (x, y) Option Tuple
    pub fn set_circles_positions(
        self,
        get_position: impl Fn(usize, f32) -> (Option<f32>, Option<f32>),
    ) -> Self {
        Self {
            circles: self
                .circles
                .into_iter()
                .enumerate()
                .map(|(i, c)| c.set_position(get_position(i, c.radius)))
                .collect(),
            speed: self.speed,
        }
    }

    // Snap each circle to the previous circle frontier
    pub fn update_positions(&mut self) {
        for i in 1..self.circles.len() {
            // Update direction towards next circle
            let target = self.circles[i - 1].position;
            self.circles[i].set_target(target);

            // Normalize the distance vector
            self.circles[i].normalize_direction();

            // Set the center of the circle on the circumference of the  previous circle
            let distance = self.circles[i - 1].radius_scale;
            self.circles[i].bound_to_target(target, distance)
        }
    }

    // Move the first circle in the direction it's pointing
    // Update the rest of the circles to follow
    pub fn move_chain(&mut self) {
        self.circles[0].position =
            self.circles[0].position + self.circles[0].direction * self.speed;
        self.update_positions();
    }
}
