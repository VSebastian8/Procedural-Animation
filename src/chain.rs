use std::f32::consts::PI;

use crate::circle::*;
use iced::Vector;

// Using angles in radians: 360 degrees == 2PI radians
pub struct Chain {
    pub circles: Vec<Circle>,
    pub speed: f32,
    max_speed: f32,
    min_speed: f32,
    pub destination: Vector,
    pub vision_angle: f32,
    rotation_speed: f32,
    locked: bool,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            circles: vec![Circle::default()],
            speed: 4.0,
            max_speed: 8.0,
            min_speed: 3.0,
            destination: [0.0, 0.0].into(),
            vision_angle: PI / 6.0,
            rotation_speed: PI / 100.0,
            locked: false,
        }
    }
}

enum Orientation {
    LEFT,
    RIGHT,
    CENTER,
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
            max_speed: self.max_speed,
            min_speed: self.min_speed,
            destination: self.destination,
            vision_angle: self.vision_angle,
            rotation_speed: self.rotation_speed,
            locked: self.locked,
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
            max_speed: self.max_speed,
            min_speed: self.min_speed,
            destination: self.destination,
            vision_angle: self.vision_angle,
            rotation_speed: self.rotation_speed,
            locked: self.locked,
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
            max_speed: self.max_speed,
            min_speed: self.min_speed,
            destination: self.destination,
            vision_angle: self.vision_angle,
            rotation_speed: self.rotation_speed,
            locked: self.locked,
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

    pub fn set_destination(&mut self, new_destination: Vector) {
        self.destination = new_destination;
        self.locked = false;
    }

    pub fn reached_destination(&self) -> bool {
        Self::vector_length(self.circles[0].position - self.destination) < self.max_speed + 10.0
    }

    // Calculate the length of a 2D vector
    fn vector_length(v: Vector) -> f32 {
        v.x.powf(2.0) + v.y.powf(2.0)
    }

    // Calculate the angle between 2 vectors
    fn angle_2_vectors(a: Vector, b: Vector) -> f32 {
        ((a.x * b.x + a.y * b.y)
            / ((a.x.powf(2.0) + a.y.powf(2.0)).sqrt() * (b.x.powf(2.0) + b.y.powf(2.0)).sqrt()))
        .acos()
    }

    // Determine wether c is Left, Right or Colinear with the vector from a to b
    fn orientation_test(a: Vector, b: Vector, c: Vector) -> Orientation {
        let det = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        if det == 0.0 {
            Orientation::CENTER
        } else if det < 0.0 {
            Orientation::LEFT
        } else {
            Orientation::RIGHT
        }
    }

    // Rotate the vector v by a degrees
    fn rotate_vector(v: Vector, a: f32) -> Vector {
        // println!("{}", a);
        [a.cos() * v.x - a.sin() * v.y, a.sin() * v.x + a.cos() * v.y].into()
    }

    // Move the first circle in the direction it's pointing
    // Update the rest of the circles to follow
    pub fn move_chain(&mut self) {
        if !Self::reached_destination(&self) {
            if self.locked
                || Self::angle_2_vectors(
                    self.circles[0].direction,
                    self.destination - self.circles[0].position,
                ) < self.vision_angle
            {
                self.circles[0].set_target(self.destination);
                self.locked = true;
                self.rotation_speed = self.vision_angle / 25.0;
                self.speed = if self.speed >= self.max_speed {
                    self.max_speed
                } else {
                    self.speed + 0.05
                };
            } else {
                self.rotation_speed += 0.0005;
                match Self::orientation_test(
                    self.circles[0].position,
                    self.circles[0].position + self.circles[0].direction * 100.0,
                    self.destination,
                ) {
                    Orientation::LEFT => {
                        self.circles[0].direction =
                            Self::rotate_vector(self.circles[0].direction, -self.rotation_speed);
                    }
                    Orientation::RIGHT | Orientation::CENTER => {
                        self.circles[0].direction =
                            Self::rotate_vector(self.circles[0].direction, self.rotation_speed);
                    }
                }
                self.speed = if self.speed <= self.min_speed {
                    self.min_speed
                } else {
                    self.speed - 0.05
                };
            }

            self.circles[0].normalize_direction();
            self.circles[0].position =
                self.circles[0].position + self.circles[0].direction * self.speed;
        }

        self.update_positions();
    }
}
