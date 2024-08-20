use iced::{
    mouse,
    widget::canvas::{Frame, Geometry, Path, Program},
    Color, Point, Rectangle, Theme, Renderer, Vector
};

pub struct Screen {
    pub position: Vector,
    pub speed: f32,
    pub radius: f32,
}

impl Screen {
    pub fn update_state(&mut self){
        self.position.x += self.speed;
        if self.position.x > 500.0 || self.position.x < -500.0 {
            self.speed *= -1.0;
        }
    }
}

impl<Message> Program<Message> for Screen {
    type State = (); // extra type we do not use, different from Screen

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        use rand::Rng;

        let mut rng = rand::thread_rng();
        // Drawing the background
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb8(2, 2, 32));
        let y_vector = Vector::new(0.0, rng.gen_range(-5.0..5.0));

        // Drawing a circle of radius 250 at it's (x, y) position
        frame.fill(
            &Path::circle(frame.center() + self.position + y_vector, self.radius),
            Color::from_rgb8(0, 179, 134),
        );

        vec![frame.into_geometry()]
    }
}
