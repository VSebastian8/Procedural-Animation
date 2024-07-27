use iced::{
    mouse,
    widget::{
        canvas::{Frame, Geometry, Path, Program},
        Canvas,
    },
    Color, Length, Point, Rectangle, Renderer, Sandbox, Settings, Theme,
};

fn main() -> iced::Result {
    AnimationApp::run(Settings::default())
}

struct AnimationApp;

impl Sandbox for AnimationApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Animal Procedural Animation")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> { 
            Canvas::new(AnimationProgram).width(Length::Fill).height(Length::Fill).into()
    }
}

struct AnimationProgram;

impl<Message> Program<Message> for AnimationProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // println!("{}", frame.center());

        // Drawing the background
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb8(2, 2, 32));

        // Drawing a circle of radius 250 in the center of the canvas
        frame.fill(
            &Path::circle(frame.center(), 250.0),
            Color::from_rgb8(0, 179, 134),
        );

        vec![frame.into_geometry()]
    }
}