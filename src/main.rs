use iced::{
    mouse,
    executor,
    time::{
        self, Duration
    },
    widget::{ 
        canvas::{Frame, Geometry, Path, Program},
        Canvas
    },
    Application, Command, Settings, Color, Length, Point, Rectangle, Theme, Renderer, Vector
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    Update,
}

struct MyApp {
    time_units: u32,
    fps: u32,
    canvas_state: CanvasState,
}

struct CanvasState {
    position: Vector,
    speed: f32,
    radius: f32,
}

impl CanvasState {
    pub fn update_state(&mut self){
        self.position.x += self.speed;
        if self.position.x > 500.0 || self.position.x < -500.0 {
            self.speed *= -1.0;
        }
    }
}

impl<Message> Program<Message> for CanvasState {
    type State = (); // extra type we do not use, different from CanvasState

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
 
impl Application for MyApp {
    type Executor = executor::Default;
    type Message = MyAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            time_units: 0, 
            fps: 30,
            canvas_state: CanvasState {
                position: [0.0, 0.0].into(),
                speed: 15.0,
                radius: 75.0,
                },
            },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("Procedural Animation")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::Update => {
                self.time_units += 1;
                if self.time_units % self.fps == 0 {
                    println!("{}", self.time_units);
                }
                self.canvas_state.update_state();
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        Canvas::new(&self.canvas_state).width(Length::Fill).height(Length::Fill).into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis((1000 / self.fps) as u64)).map(|_| MyAppMessage::Update)
    }
}