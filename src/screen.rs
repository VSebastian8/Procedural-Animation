use crate::circle::*;
use iced::{
    mouse,
    widget::canvas::{Cache, Geometry, Program, Stroke},
    Color, Point, Rectangle, Renderer, Theme,
};

pub struct Screen {
    cache: Cache,
    pub circle: Circle,
    pub speed: f32,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
            circle: Circle::default(),
            speed: 15.0,
        }
    }

    pub fn update(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        self.circle.position.x += self.speed;
        self.circle.position.y = rng.gen_range(-2.0..2.0);

        if self.circle.position.x > 500.0 || self.circle.position.x < -500.0 {
            self.speed *= -1.0;
        }
        // Clear the cache to redraw the canvas
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
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb8(2, 2, 32));

            // Drawing a circle at it's (x, y) position + y_offset
            frame.fill(&self.circle.path(frame.center()), self.circle.color);
            frame.stroke(
                &self.circle.path(frame.center()),
                Stroke {
                    style: Color::WHITE.into(),
                    width: 4.0,
                    ..Default::default()
                },
            );
        });

        vec![geometry]
    }
}
