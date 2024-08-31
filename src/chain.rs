use crate::circle::*;
use iced::Vector;
use std::f32::consts::PI;

// Using angles in radians: 360 degrees == 2PI radians
pub struct Chain {
    pub circles: Vec<Circle>,
    pub destination: Vector,
    pub vision_angle: f32,
    rotation_speed: f32,
    pub speed: f32,
    min_speed: f32,
    max_speed: f32,
    locked: bool,
}

// Builder pattern for chain struct
pub struct ChainBuilder {
    circles: Vec<Circle>,
    destination: Option<Vector>,
    vision_angle: Option<f32>,
    min_speed: Option<f32>,
    max_speed: Option<f32>,
}

#[allow(dead_code)]
impl ChainBuilder {
    // Function that sets the radius for each circle
    pub fn circles_radii(&mut self, radius_array: Vec<f32>) -> &mut Self {
        self.circles = radius_array
            .into_iter()
            .map(|r| Circle::default().set_radius(r))
            .collect();
        self
    }

    // Set the circle positions according to a function given as argument
    // The function takes a circle's index and radius and returns an (x, y) Option Tuple
    pub fn circles_positions(
        &mut self,
        get_position: impl Fn(usize, f32) -> (Option<f32>, Option<f32>),
    ) -> &mut Self {
        self.circles = self
            .circles
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, c)| c.set_position(get_position(i, c.radius)))
            .collect();
        self
    }

    // Set the target destination
    pub fn destination(&mut self, destination: Vector) -> &mut Self {
        self.destination = Some(destination);
        self
    }

    // Set the vision angle for turning
    pub fn vision_angle(&mut self, vision_angle: f32) -> &mut Self {
        self.vision_angle = Some(vision_angle);
        self
    }

    // Set the min and max speeds
    pub fn speed_bounds(&mut self, min_speed: f32, max_speed: f32) -> &mut Self {
        self.min_speed = Some(min_speed);
        self.max_speed = Some(max_speed);
        self
    }

    // Return the built chain
    pub fn build(&mut self) -> Chain {
        Chain {
            circles: self.circles.clone(),
            destination: self.destination.unwrap_or(Vector::new(0.0, 0.0)).clone(),
            vision_angle: self.vision_angle.unwrap_or(PI / 6.0),
            rotation_speed: self.vision_angle.unwrap_or(PI / 6.0) / 25.0,
            speed: self.min_speed.unwrap_or(3.0),
            min_speed: self.min_speed.unwrap_or(3.0),
            max_speed: self.max_speed.unwrap_or(7.0),
            locked: false,
        }
    }
}

enum Orientation {
    LEFT,
    CENTER,
    RIGHT,
}

impl Chain {
    pub fn new() -> ChainBuilder {
        ChainBuilder {
            circles: vec![Circle::default()],
            destination: None,
            vision_angle: None,
            min_speed: None,
            max_speed: None,
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
            let distance = -self.circles[i - 1].radius;
            self.circles[i].bound_to_target(target, distance);
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
        Vector::new(a.cos() * v.x - a.sin() * v.y, a.sin() * v.x + a.cos() * v.y)
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
