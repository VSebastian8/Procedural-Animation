use rand::Rng;
use std::collections::VecDeque;

use crate::util::*;
use iced::{
    widget::canvas::{Frame, Path},
    Color, Point,
};

#[allow(dead_code)]
pub enum ProviderStrategy {
    Random,
    OtherHalf,
    MouseClick,
}

pub struct PointProvider {
    points: VecDeque<Point>,
    strategy: ProviderStrategy,
    width: f32,
    height: f32,
}

impl PointProvider {
    pub fn new(strategy: ProviderStrategy, width: f32, height: f32) -> Self {
        PointProvider {
            points: VecDeque::new(),
            strategy,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn add(&mut self, point: Point) {
        self.points.push_back(point);
    }

    pub fn remove(&mut self, point: Point) {
        let radius: f32 = 10.0;
        self.points = self
            .points
            .iter()
            .filter(|p| point_distance(point, **p) > radius)
            .map(|p| *p)
            .collect();
    }

    // Get the next destination
    pub fn next(&mut self, position: Point) -> Point {
        let mut rng = rand::thread_rng();
        match self.points.pop_front() {
            Some(point) => point,
            None => match self.strategy {
                ProviderStrategy::Random | ProviderStrategy::MouseClick => Point::new(
                    rng.gen_range((-self.width / 2.0)..(self.width / 2.0)),
                    rng.gen_range((-self.height / 2.0)..(self.height / 2.0)),
                ),
                ProviderStrategy::OtherHalf => {
                    if position.x < 0.0 {
                        Point::new(
                            rng.gen_range(0.0..(self.width / 2.0)),
                            rng.gen_range((-self.height / 2.0)..(self.height / 2.0)),
                        )
                    } else {
                        Point::new(
                            rng.gen_range((-self.width / 2.0)..0.0),
                            rng.gen_range((-self.height / 2.0)..(self.height / 2.0)),
                        )
                    }
                }
            },
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        // Draw the points in the queue
        self.points.iter().zip(1..).for_each(|(point, index)| {
            frame.fill(
                &Path::circle(*point + point_to_vector(frame.center()), 5.0),
                Color::from_rgba8(195, 192, 255, f32::max(0.2, 1.0 - 0.1 * index as f32)),
            )
        });
    }
}
