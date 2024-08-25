use iced::{
    Application, // Also import implemented Traits
    Result,
    Settings,
};
mod app;
mod chain;
mod circle;
mod screen;
use crate::app::*;

fn main() -> Result {
    MyApp::run(Settings::default())
}
