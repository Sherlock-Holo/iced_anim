use std::f32::consts::FRAC_PI_2;

use cosmic::{
    iced::{
        mouse,
        widget::canvas::{self, Frame, Geometry, Path, Stroke},
        Color, Element,
        Length::Fill,
        Point, Rectangle, Theme,
    },
    widget::Canvas,
    Renderer,
};
use iced_anim::{spring::Motion, Animated, Animation, Event};

#[derive(Debug, Clone)]
enum Message {
    Rotate,
    UpdateAngle(Event<f32>),
}

struct State {
    /// Animated rotation angle in radians.
    angle: Animated<f32>,
    /// How many 90° increments to rotate.
    clicks: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            angle: Animated::spring(0.0, Motion::default()),
            clicks: 0,
        }
    }
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Rotate => {
                self.clicks += 1;
                self.angle.set_target(self.clicks as f32 * FRAC_PI_2);
            }
            Message::UpdateAngle(event) => self.angle.update(event),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        cosmic::iced::widget::mouse_area(
            Animation::new(&self.angle, Canvas::new(self).width(Fill).height(Fill))
                .on_update(Message::UpdateAngle),
        )
        .on_press(Message::Rotate)
        .into()
    }
}

impl canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        let center = bounds.center();
        let size = 120.0_f32;
        let half = size / 2.0;

        // Triangle pointing right, like "▶"
        let points = [
            Point::new(center.x + half, center.y),
            Point::new(center.x - half, center.y - half),
            Point::new(center.x - half, center.y + half),
        ];

        // Rotate around center
        let angle = *self.angle.value();
        let (sin, cos) = angle.sin_cos();
        let rotated: [Point; 3] = points.map(|p| {
            let dx = p.x - center.x;
            let dy = p.y - center.y;
            Point::new(
                center.x + dx * cos - dy * sin,
                center.y + dx * sin + dy * cos,
            )
        });

        let path = Path::new(|b| {
            b.move_to(rotated[0]);
            b.line_to(rotated[1]);
            b.line_to(rotated[2]);
            b.close();
        });

        frame.fill(&path, Color::from_rgb(0.2, 0.6, 1.0));
        frame.stroke(
            &path,
            Stroke::default()
                .with_color(Color::from_rgb(0.0, 0.3, 0.7))
                .with_width(3.0),
        );

        vec![frame.into_geometry()]
    }
}

pub fn main() -> cosmic::iced::Result {
    cosmic::iced::application(State::default, State::update, State::view)
        .title("Rotating Triangle")
        .run()
}
