use crate::point_provider::*;
use crate::snake::*;
use iced::mouse::Button;
use iced::time::{self, Duration};
use iced::widget::canvas::event::{self, Event};
use iced::widget::Canvas;
use iced::Length;
use iced::{
    mouse,
    widget::canvas::{Cache, Geometry, Program},
    Color, Point, Rectangle, Renderer, Theme,
};
use iced_runtime::window::resize_events;
use std::cell::RefCell;
use std::rc::Rc;

// State of the screen
pub struct Screen {
    fps: u32,
    cache: Cache,
    snake: Snake,
    point_provider: Rc<RefCell<PointProvider>>,
}

// Event messages
#[derive(Debug)]
pub enum ScreenMessage {
    Update,
    Resized(f32, f32),
    AddPoint(Point),
}

impl Default for Screen {
    fn default() -> Self {
        let pp = Rc::new(RefCell::new(PointProvider::new(
            ProviderStrategy::OtherHalf,
            800.0,
            600.0,
        )));
        Self {
            fps: 30,
            cache: Cache::new(),
            snake: Snake::new(Rc::clone(&pp)),
            point_provider: pp,
        }
    }
}

// Handle Events
pub fn update(screen: &mut Screen, message: ScreenMessage) {
    match message {
        ScreenMessage::Update => {
            screen.snake.update();
            screen.cache.clear();
        }
        ScreenMessage::Resized(width, height) => {
            screen.point_provider.borrow_mut().resize(width, height);
        }
        ScreenMessage::AddPoint(p) => {
            screen.point_provider.borrow_mut().add(p);
        }
    }
}

// Create the canvas
pub fn view<'a>(screen: &'a Screen) -> iced::Element<'a, ScreenMessage> {
    Canvas::new(screen)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// Frame every 1/fps seconds
pub fn subscription(screen: &Screen) -> iced::Subscription<ScreenMessage> {
    iced::Subscription::batch(vec![
        time::every(Duration::from_millis((1000 / screen.fps) as u64))
            .map(|_| ScreenMessage::Update),
        resize_events().map(|(_, size)| ScreenMessage::Resized(size.width, size.height)),
    ])
}

// Drawing the canvas
impl Program<ScreenMessage> for Screen {
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
            // Draw the background
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb8(39, 45, 52));
            // Draw the queued points
            self.point_provider.borrow().draw(frame);
            // Draw the animal
            self.snake.draw(frame);
        });

        vec![geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<ScreenMessage>) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(Button::Left)) => match cursor {
                mouse::Cursor::Available(p) => {
                    let s = bounds.size();
                    (
                        event::Status::Captured,
                        Some(ScreenMessage::AddPoint(Point {
                            x: p.x - s.width / 2.0,
                            y: p.y - s.height / 2.0,
                        })),
                    )
                }
                mouse::Cursor::Unavailable => (event::Status::Ignored, None),
            },
            _ => (event::Status::Ignored, None),
        }
    }
}
