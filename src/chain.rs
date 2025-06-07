use std::{cmp::min, f32::consts::PI};

use crate::circle::*;
use crate::util::*;
use iced::{widget::canvas::Path, Point};

// Using angles in radians: 360 degrees == 2PI radians
pub struct Chain {
    pub circles: Vec<Circle>,
    pub outlines: Vec<Vec<f32>>,
}

// Builder pattern for chain struct
pub struct ChainBuilder {
    circles: Vec<Circle>,
    outlines: Vec<Vec<f32>>,
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

    // Function puts a marker on the middle of each half of each circle
    pub fn default_outline(&mut self) -> &mut Self {
        let size = self.circles.len();
        self.outlines = vec![vec![PI / 2.0; 1]; size];
        self.outlines.extend(vec![vec![3.0 * PI / 2.0; 1]; size]);
        self
    }

    // Add the angles from the angles vector, while splitting them into left and right and then ordering the outlines vector for each circle
    // The function ignores Nan floats and also simplifies the angle (with support negative angles up to -2PI)
    pub fn refine_outline(&mut self, ang: Vec<Vec<f32>>) -> &mut Self {
        let size = self.circles.len();
        // Filter the input parameter to ignore Nans and to take the modulo of angles higher than PI * 2
        let angles: Vec<Vec<f32>> = ang
            .iter()
            .map(|a| {
                a.iter()
                    .filter(|f| !f.is_nan())
                    .map(|f| *f)
                    .map(|f| (f + PI * 2.0) % (PI * 2.0))
                    .collect()
            })
            .collect();
        let asize = angles.len();
        for i in 0..min(size, asize) {
            // Filter the angles into the two halves
            if !angles[i].is_empty() {
                self.outlines[i] = angles[i].iter().filter(|f| **f < PI).map(|f| *f).collect();
                self.outlines[size + i] =
                    angles[i].iter().filter(|f| **f >= PI).map(|f| *f).collect();
            }
            // Sort the angles for each half
            self.outlines[i].sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            self.outlines[size + i]
                .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        }
        self
    }

    // Return the built chain
    pub fn build(&mut self) -> Chain {
        Chain {
            circles: self.circles.clone(),
            outlines: self.outlines.clone(),
        }
    }
}

impl Chain {
    pub fn new() -> ChainBuilder {
        ChainBuilder {
            circles: vec![Circle::default()],
            outlines: Vec::new(),
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

    // Function to return a path of the Chain
    pub fn circle_path(&self, frame_center: Point) -> Path {
        Path::new(|builder| {
            for i in 0..self.circles.len() {
                // builder.circle(&circle.path(frame.center()),
                builder.circle(
                    self.circles[i].position + point_to_vector(frame_center),
                    self.circles[i].radius,
                );
            }

            builder.close();
        })
    }

    pub fn outline_path(&self, frame_center: Point) -> Path {
        Path::new(|builder| {
            let n = self.circles.len();
            // Start the path at the last point of the right half of the first line
            builder.move_to(
                self.circles[0].point_on_circle(rotate_vector(
                    self.circles[0].direction,
                    *self.outlines[n].last().unwrap_or(&0.0),
                )) + point_to_vector(frame_center),
            );

            for i in 0..n {
                for ang in self.outlines[i].iter() {
                    builder.line_to(
                        self.circles[i]
                            .point_on_circle(rotate_vector(self.circles[i].direction, *ang))
                            + point_to_vector(frame_center),
                    );
                }
            }
            for i in (0..n).rev() {
                for ang in self.outlines[i + n].iter() {
                    builder.line_to(
                        self.circles[i]
                            .point_on_circle(rotate_vector(self.circles[i].direction, *ang))
                            + point_to_vector(frame_center),
                    );
                }
            }
            builder.close();
        })
    }
}
