use crate::screen::*;
use iced::{
    executor,
    time::{self, Duration},
    widget::Canvas,
    Application, Command, Length,
};

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    Update,
}
pub struct MyApp {
    fps: u32,
    screen: Screen,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = MyAppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                fps: 30,
                screen: Screen::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Procedural Animation")
    }

    // Call the update screen function each frame
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyAppMessage::Update => {
                self.screen.update();
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        Canvas::new(&self.screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // Frame every 1/fps seconds
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(Duration::from_millis((1000 / self.fps) as u64)).map(|_| MyAppMessage::Update)
    }
}
