use iced::{
    mouse,
    widget::canvas::{Cache, Geometry, Path, Program},
    Color, Point, Rectangle, Renderer, Theme, Vector,
};

pub struct Screen {
    cache: Cache,
    pub position: Vector,
    pub speed: f32,
    pub radius: f32,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
            position: [0.0, 0.0].into(),
            speed: 15.0,
            radius: 75.0,
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.speed;
        if self.position.x > 500.0 || self.position.x < -500.0 {
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
        use rand::Rng;

        // Closure gets updated only when we clear the cache in update()
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let mut rng = rand::thread_rng();
            let y_offset = Vector::new(0.0, rng.gen_range(-5.0..5.0));

            // Drawing the background
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb8(2, 2, 32));

            // Drawing a circle at it's (x, y) position + y_offset
            frame.fill(
                &Path::circle(frame.center() + self.position + y_offset, self.radius),
                Color::from_rgb8(0, 179, 134),
            );
        });

        vec![geometry]
    }
}
