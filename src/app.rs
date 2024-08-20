use iced::{
    executor,
    time::{
        self, Duration
    },
    widget:: Canvas,
    Application, Command, Length
};
use crate::screen::*;

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    Update,
}

pub struct MyApp {
    time_units: u32,
    fps: u32,
    canvas_state: Screen,
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
            canvas_state: Screen {
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
