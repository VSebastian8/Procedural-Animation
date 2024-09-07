use crate::snake::*;

use iced::{
    mouse,
    widget::canvas::{Cache, Geometry, Program},
    Color, Point, Rectangle, Renderer, Theme,
};

pub struct Screen {
    cache: Cache,
    snake: Snake,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
            snake: Snake::new(),
        }
    }

    pub fn update(&mut self, t: u32) {
        self.snake.update((t / 20) % 3);

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

            self.snake.draw(frame);
        });

        vec![geometry]
    }
}
