use std::cmp::min;

use crate::circle::*;
use iced::{widget::canvas::Path, Point, Vector};

// Using angles in radians: 360 degrees == 2PI radians
pub struct Chain {
    pub circles: Vec<Circle>,
}

// Builder pattern for chain struct
pub struct ChainBuilder {
    circles: Vec<Circle>,
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

    // Return the built chain
    pub fn build(&mut self) -> Chain {
        Chain {
            circles: self.circles.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Orientation {
    LEFT,
    CENTER,
    RIGHT,
}

impl Chain {
    pub fn new() -> ChainBuilder {
        ChainBuilder {
            circles: vec![Circle::default()],
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

    // Calculate the length of a 2D vector
    pub fn vector_length(v: Vector) -> f32 {
        (v.x.powf(2.0) + v.y.powf(2.0)).sqrt()
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

    #[allow(dead_code)]
    // Function that calculates the circle passing through 3 points, returns circle center and radius
    pub fn circle_from_three_points(a: Vector, b: Vector, c: Vector) -> (Vector, f32) {
        let xab = a.x - b.x;
        let xac = a.x - c.x;
        let yab = a.y - b.y;
        let yac = a.y - c.y;

        // Square difference
        let sqxac = a.x * a.x - c.x * c.x;
        let sqxba = b.x * b.x - a.x * a.x;
        let sqyac = a.y * a.y - c.y * c.y;
        let sqyba = b.y * b.y - a.y * a.y;

        let f = (sqxac * xab + sqyac * xab + sqxba * xac + sqyba * xac)
            / (2.0 * (yab * xac - yac * xab));
        let g = (sqxac * yab + sqyac * yab + sqxba * yac + sqyba * yac)
            / (2.0 * (xab * yac - xac * yab));
        let c = -a.x * a.x - a.y * a.y - 2.0 * g * a.x - 2.0 * f * a.y;
        let r = (f * f + g * g - c).sqrt();

        (Vector::new(-g, -f), r)
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
