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
            .circles_radii(vec![
                30.0, 48.0, 80.0, 55.5, 40.0, 30.5, 20.0, 20.0, 20.0, 20.0, 25.5,
            ])
            .circles_positions(|i: usize, r: f32| {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                (
                    Some(i as f32 * r * 3.0 + 100.0),
                    Some(rng.gen_range(-300.0..300.0)),
                )
            })
            .destination(Vector::new(-250.0, 0.0))
            .build();
        chain.update_positions();
        Self {
            cache: Cache::new(),
            chain,
        }
    }

    pub fn update(&mut self) {
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

            // Drawing a circle at it's (x, y) position + y_offset
            for circle in &self.chain.circles {
                frame.stroke(
                    &circle.path(frame.center()),
                    Stroke {
                        style: Color::WHITE.into(),
                        width: 4.0,
                        ..Default::default()
                    },
                );
                if circle.show_center {
                    frame.fill(&circle.center_path(frame.center()), Color::WHITE);
                }
            }

            // Draw the target
            frame.fill(
                &Path::circle(frame.center() + self.chain.destination, 5.0),
                Color::from_rgb8(220, 10, 120),
            );
        });

        vec![geometry]
    }
}
