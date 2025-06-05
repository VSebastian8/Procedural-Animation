use iced::Result;
mod chain;
mod circle;
mod point_provider;
mod screen;
mod snake;
use crate::screen::*;

fn main() -> Result {
    iced::application("Procedural Animation", update, view)
        .subscription(subscription)
        .run()
}
