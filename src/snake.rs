use crate::chain::*;
use iced::{
    widget::canvas::{Frame, Path, Stroke},
    Color, Point, Vector,
};
use std::f32::consts::PI;

pub struct Snake {
    pub chain: Chain,
    pub color: Color,
}

impl Snake {
    pub fn new() -> Self {
        let mut chain = Chain::new()
            .circles_radii(vec![30.0, 48.0, 70.0, 60.5, 40.0, 30.5, 20.0, 20.0, 25.5])
            .circles_offsets(vec![0.0, 0.0, 13.0, -20.2, -10.0, -10.0, 15.0, 30.0, 0.0])
            .circles_positions(|i: usize, r: f32| {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                (
                    Some(i as f32 * r * 3.0 + 100.0),
                    Some(rng.gen_range(-300.0..300.0)),
                )
            })
            .vision_angle(PI / 10.0)
            .destination(Vector::new(-250.0, 0.0))
            .build();
        chain.update_positions(0);
        Snake {
            chain,
            color: Color::from_rgb8(168, 58, 50),
        }
    }

    pub fn update(&mut self, tail_moment: u32) {
        // Every now and then, set the chain destination to a random point
        if self.chain.reached_destination() {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            self.chain.set_destination(Vector::new(
                rng.gen_range(-400.0..400.0),
                rng.gen_range(-300.0..300.0),
            ))
        }

        // Move the first circle and readjust all the rest
        self.chain.move_chain();

        match tail_moment {
            0 => {
                self.chain.grow_tail();
            }
            1 => {
                self.chain.shrink_tail();
            }
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        // Draw the target
        frame.fill(
            &Path::circle(frame.center() + self.chain.destination, 5.0),
            self.color,
        );

        // Draw the chain outline
        frame.stroke(
            &self.chain.path(frame.center()),
            Stroke {
                style: Color::WHITE.into(),
                width: 4.0,
                ..Default::default()
            },
        );
        // Color the chain
        frame.fill(&self.chain.path(frame.center()), self.color);

        frame.fill(&self.eyes_path(frame.center()), Color::WHITE)
    }

    // Function for drawing the snake's eyes
    pub fn eyes_path(&self, frame_center: Point) -> Path {
        Path::new(|builder| {
            builder.circle(
                frame_center
                    + self.chain.circles[0].position
                    + Chain::rotate_vector(self.chain.circles[0].direction, -PI / 4.0)
                        * self.chain.circles[0].radius
                        * 0.75,
                5.0,
            );
            builder.circle(
                frame_center
                    + self.chain.circles[0].position
                    + Chain::rotate_vector(self.chain.circles[0].direction, PI / 4.0)
                        * self.chain.circles[0].radius
                        * 0.75,
                5.0,
            );
            builder.close();
        })
    }
}
