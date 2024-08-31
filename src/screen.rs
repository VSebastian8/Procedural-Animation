use std::f32::consts::PI;

use crate::chain::*;
use iced::{
    mouse,
    widget::canvas::{Cache, Geometry, Path, Program, Stroke},
    Color, Point, Rectangle, Renderer, Theme, Vector,
};

pub struct Screen {
    cache: Cache,
    pub chain: Chain,
}

impl Screen {
    pub fn new() -> Self {
        let mut chain = Chain::new()
            .circles_radii(vec![30.0, 48.0, 80.0, 55.5, 40.0, 30.5, 20.0, 20.0, 25.5])
            .circles_offsets(vec![0.0, 0.0, 13.0, -20.2, 0.0, 0.0, 20.0, 30.0, 0.0])
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
        chain.update_positions();
        Self {
            cache: Cache::new(),
            chain,
        }
    }

    pub fn update(&mut self, t: u32) {
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

        match (t / 20) % 3 {
            0 => {
                self.chain.grow_tail();
            }
            1 => {
                self.chain.shrink_tail();
            }
            _ => {}
        }

        // // Clear the cache to redraw the canvas
        self.cache.clear();
    }
}

impl<Message> Program<Message> for Screen {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        // Closure gets updated only when we clear the cache in update()
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            // Drawing the background
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb8(39, 45, 52));

            // Draw the target
            frame.fill(
                &Path::circle(frame.center() + self.chain.destination, 5.0),
                Color::from_rgb8(220, 10, 120),
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
            frame.fill(
                &self.chain.path(frame.center()),
                Color::from_rgb8(54, 125, 163),
            );

            frame.fill(&self.chain.eyes_path(frame.center()), Color::WHITE)
        });

        vec![geometry]
    }
}
