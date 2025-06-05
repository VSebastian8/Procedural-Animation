use rand::Rng;
use std::collections::VecDeque;

use iced::Vector;

#[allow(dead_code)]
pub enum ProviderStrategy {
    Random,
    OtherHalf,
}

pub struct PointProvider {
    points: VecDeque<Vector>,
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

    // Get the next destination
    pub fn next(&mut self, position: Vector) -> Vector {
        let mut rng = rand::thread_rng();
        match self.points.pop_front() {
            Some(point) => point,
            None => match self.strategy {
                ProviderStrategy::Random => Vector::new(
                    rng.gen_range((-self.width / 2.0)..(self.width / 2.0)),
                    rng.gen_range((-self.height / 2.0)..(self.height / 2.0)),
                ),
                ProviderStrategy::OtherHalf => {
                    if position.x < 0.0 {
                        Vector::new(
                            rng.gen_range(0.0..(self.width / 2.0)),
                            rng.gen_range((-self.height / 2.0)..(self.height / 2.0)),
                        )
                    } else {
                        Vector::new(
                            rng.gen_range((-self.width / 2.0)..0.0),
                            rng.gen_range((-self.height / 2.0)..(self.height / 2.0)),
                        )
                    }
                }
            },
        }
    }
}
