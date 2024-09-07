use crate::circle::*;
use iced::{widget::canvas::Path, Point, Vector};
use std::{cmp::min, f32::consts::PI};

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
            .map(|r| Circle::default().set_radius(r).clone())
            .collect();
        self
    }

    // Function that sets the offset from the frontier for each circle
    pub fn circles_offsets(&mut self, offset_array: Vec<f32>) -> &mut Self {
        for i in 0..min(offset_array.len(), self.circles.len()) {
            self.circles[i].offset = offset_array[i];
        }
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

pub enum Orientation {
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

    // Function to snap circle i to the frontier of circle j
    pub fn bind_circle(&mut self, i: usize, j: usize) {
        // Update direction towards next circle
        let target = self.circles[j].position;
        self.circles[i].set_target(target);

        // Normalize the distance vector
        self.circles[i].normalize_direction();

        // Set the center of the circle on the circumference of the  previous circle
        let distance = self.circles[j].radius;
        self.circles[i].bound_to_target(target, distance);
    }

    // Starting from the k circle, snap each circle in either direction
    pub fn update_positions(&mut self, k: usize) {
        for i in (0..k).rev() {
            self.bind_circle(i, i + 1);
        }
        for i in (k + 1)..self.circles.len() {
            self.bind_circle(i, i - 1);
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
    pub fn vector_length(v: Vector) -> f32 {
        v.x.powf(2.0) + v.y.powf(2.0)
    }

    // Calculate the angle between 2 vectors
    pub fn angle_2_vectors(a: Vector, b: Vector) -> f32 {
        ((a.x * b.x + a.y * b.y)
            / ((a.x.powf(2.0) + a.y.powf(2.0)).sqrt() * (b.x.powf(2.0) + b.y.powf(2.0)).sqrt()))
        .acos()
    }

    // Determine wether c is Left, Right or Colinear with the vector from a to b
    pub fn orientation_test(a: Vector, b: Vector, c: Vector) -> Orientation {
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
    pub fn rotate_vector(v: Vector, a: f32) -> Vector {
        Vector::new(a.cos() * v.x - a.sin() * v.y, a.sin() * v.x + a.cos() * v.y)
    }

    // Function for heading straight for the point once the target is in the field of vision
    pub fn go_straight(&mut self) {
        self.circles[0].set_target(self.destination);
        self.locked = true;
        self.rotation_speed = self.vision_angle / 25.0;
        self.speed = if self.speed >= self.max_speed {
            self.max_speed
        } else {
            self.speed + 0.05
        };
        self.circles[0].set_target(self.destination);
        self.locked = true;
        self.rotation_speed = self.vision_angle / 25.0;
        self.speed = if self.speed >= self.max_speed {
            self.max_speed
        } else {
            self.speed + 0.05
        };
    }

    // Turn at {rotation_speed} when target is not in the field of vision
    pub fn turn(&mut self) {
        self.rotation_speed += 0.0002;
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

    // Function for changing the position of the first circle
    pub fn move_head(&mut self) {
        // Modify the first circle's direction depending on the target's direction
        if self.locked
            || Self::angle_2_vectors(
                self.circles[0].direction,
                self.destination - self.circles[0].position,
            ) < self.vision_angle
        {
            Self::go_straight(self);
        } else {
            Self::turn(self);
        }

        // Move the first circle in the direction it's pointing
        self.circles[0].normalize_direction();
        self.circles[0].position =
            self.circles[0].position + self.circles[0].direction * self.speed;
    }

    // Update the rest of the circles to follow the first one
    pub fn move_chain(&mut self) {
        if !Self::reached_destination(&self) {
            Self::move_head(self);
        }
        self.update_positions(0);
    }

    pub fn grow_tail(&mut self) {
        let size = self.circles.len() - 1;
        self.circles[size].radius += 0.4;
        self.circles[size - 1].radius += 0.2;
    }

    pub fn shrink_tail(&mut self) {
        let size = self.circles.len() - 1;
        self.circles[size].radius -= 0.4;
        self.circles[size - 1].radius -= 0.2;
    }

    // Function to return a path of the Chain
    pub fn path(&self, frame_center: Point) -> Path {
        Path::new(|builder| {
            for i in 0..self.circles.len() {
                // builder.circle(&circle.path(frame.center()),
                builder.circle(
                    frame_center + self.circles[i].position,
                    self.circles[i].radius,
                );
            }

            builder.close();
        })
    }
}
