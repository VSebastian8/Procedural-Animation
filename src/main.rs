use iced::{
    Application, // Also import implemented Traits
    Settings,
    Result,
};
mod screen;
mod app;
use crate::app::*;

fn main() -> Result {
    MyApp::run(Settings::default())
}
