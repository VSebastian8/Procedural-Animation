use crate::chain::*;
use iced::{
    mouse,
    widget::canvas::{Cache, Geometry, Program, Stroke},
    Color, Point, Rectangle, Renderer, Theme,
};

pub struct Screen {
    cache: Cache,
    pub chain: Chain,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
            chain: Chain::new(8),
        }
    }

    pub fn update(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for circle in &mut self.chain.circles {
            let offset: f32 = rng.gen_range(-5.0..5.0);
            circle.position.y += offset * (if rng.gen::<bool>() { -1.0 } else { 1.0 });
            circle.position.x += offset * (if rng.gen::<bool>() { -1.0 } else { 1.0 });
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

            // Drawing a circle at it's (x, y) position + y_offset
            for circle in &self.chain.circles {
                // frame.fill(&circle.path(frame.center()), Color::BLACK);
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
        });

        vec![geometry]
    }
}
