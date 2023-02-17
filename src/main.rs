use iced::widget::{button, column, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
struct Example {
    drag_test: drag_test::State,
}

impl Sandbox for Example {
    type Message = ();

    fn new() -> Self {
        Example::default()
    }

    fn title(&self) -> String {
        String::from("Drag test")
    }

    fn update(&mut self, message: Self::Message) {
        self.drag_test.request_redraw();
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            self.drag_test.view().map(|_|{()}),
        ]
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .into()
    }
}

mod drag_test {
    use iced::Color;
    use iced::mouse;
    use iced::widget::canvas::event::{self, Event};
    use iced::widget::canvas::{
        self, Canvas, Cursor, Frame, Geometry, Path, Stroke, Text,
    };
    use iced::{Element, Length, Point, Rectangle, Theme};
    use crate::drag_test;

    #[derive(Default, Debug)]
    pub struct State {
        cache: canvas::Cache,
        cursor: Option<Cursor>,
        dragging: bool,
    }

    impl State {
        pub fn view<'a>(&'a self) -> Element<'a, ()> {
            Canvas::new(Bezier { state: self })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }

        pub fn request_redraw(&mut self) {
            self.cache.clear()
        }
    }

    struct Bezier<'a> {
        state: &'a State,
    }

    impl<'a> canvas::Program<()> for Bezier<'a> {
        type State = State;

        fn update(
            &self,
            state: &mut Self::State,
            event: Event,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> (event::Status, Option<()>) {
            state.cursor.replace(cursor);

            return match event {
                Event::Mouse(mouse_event) => {
                    let message: Option<()> = match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            state.dragging = true;
                            Some(())
                        }
                        mouse::Event::CursorMoved { .. } => {
                            Some(())
                        }
                        mouse::Event::ButtonReleased(mouse::Button::Left) => {
                            state.dragging = false;
                            Some(())
                        }
                        mouse::Event::CursorLeft { .. } => {
                            Some(())
                        }
                        _ => None,
                    };

                    (event::Status::Captured, message)
                }
                _ => (event::Status::Ignored, None),
            };
        }

        fn draw(
            &self,
            state: &Self::State,
            _theme: &Theme,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> Vec<Geometry> {
            let content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                let text = Text::from(format!("{:#?}", state));
                frame.fill_text(text);

                if state.dragging {
                    frame.stroke(
                        &Path::rectangle(Point::ORIGIN, frame.size()),
                        Stroke::default().with_width(10.0).with_color(Color::from_rgba8(255, 0, 0, 1.)),
                    );
                }

                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default().with_width(2.0),
                );
            });

            vec![content]
        }

        fn mouse_interaction(
            &self,
            _state: &Self::State,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> mouse::Interaction {
            if cursor.is_over(&bounds) {
                mouse::Interaction::Crosshair
            } else {
                mouse::Interaction::default()
            }
        }
    }
}
