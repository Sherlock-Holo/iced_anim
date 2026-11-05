use std::time::Duration;

use cosmic::iced::{
    theme::palette::{self, Extended, Pair},
    widget::{column, container, pick_list, row, space, text, tooltip, Row},
    Border, Element, Length, Theme,
};
use iced_anim::{Animated, Animation, Easing, Event};

#[derive(Debug, Clone)]
enum Message {
    /// Updates the current theme.
    UpdateTheme(Event<Theme>),
}

struct State {
    /// The theme used by the application.
    theme: Animated<Theme>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Animated::new(
                Theme::Light,
                Easing::EASE.with_duration(Duration::from_millis(300)),
            ),
        }
    }
}

impl State {
    /// Gets the current theme, if any.
    fn theme(&self) -> Theme {
        self.theme.value().clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateTheme(event) => {
                self.theme.update(event);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let page = container(
            row![
                pick_list(Theme::ALL, Some(self.theme.target()), |theme| {
                    Message::UpdateTheme(theme.into())
                }),
                column![
                    column![
                        text("Palette colors").size(20),
                        palette_grid(self.theme.value().extended_palette()),
                    ]
                    .spacing(4),
                    column![
                        text("Background colors").size(20),
                        background_palette_grid(&self.theme.value().extended_palette().background),
                    ]
                    .spacing(4),
                ]
                .spacing(16)
            ]
            .spacing(8),
        )
        .padding(8)
        .style(move |theme: &Theme| container::Style {
            background: Some(theme.palette().background.into()),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill);

        Animation::new(&self.theme, page)
            .on_update(Message::UpdateTheme)
            .into()
    }
}

fn palette_grid<'a>(palette: &Extended) -> Element<'a, Message> {
    // The various shades of a palette
    let shades = [
        (
            "Primary",
            palette.primary.strong,
            palette.primary.base,
            palette.primary.weak,
        ),
        (
            "Secondary",
            palette.secondary.strong,
            palette.secondary.base,
            palette.secondary.weak,
        ),
        (
            "Success",
            palette.success.strong,
            palette.success.base,
            palette.success.weak,
        ),
        (
            "Warning",
            palette.warning.strong,
            palette.warning.base,
            palette.warning.weak,
        ),
        (
            "Danger",
            palette.danger.strong,
            palette.danger.base,
            palette.danger.weak,
        ),
    ];

    container(Row::with_children(shades.map(
        |(name, strong, base, weak)| {
            column![
                pair_square(format!("{name} weak"), weak),
                pair_square(format!("{name} base"), base),
                pair_square(format!("{name} strong"), strong),
            ]
            .into()
        },
    )))
    .padding(1.0)
    .style(|theme: &cosmic::iced::Theme| container::Style {
        border: Border {
            color: theme.palette().text,
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn background_palette_grid(background: &palette::Background) -> Element<'_, Message> {
    container(column![
        row![
            pair_square("Weak Background".to_string(), background.weak),
            pair_square("Weaker Background".to_string(), background.weaker),
            pair_square("Weakest Background".to_string(), background.weakest),
            pair_square("Base Background".to_string(), background.base),
        ],
        row![
            pair_square("Neutral Background".to_string(), background.neutral),
            pair_square("Strong Background".to_string(), background.strong),
            pair_square("Stronger Background".to_string(), background.stronger),
            pair_square("Strongest Background".to_string(), background.strongest),
        ],
    ])
    .padding(1.0)
    .style(|theme: &cosmic::iced::Theme| container::Style {
        border: Border {
            color: theme.palette().text,
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn pair_square<'a>(name: String, pair: Pair) -> Element<'a, Message> {
    tooltip(
        container(space().width(Length::Fill).height(Length::Fill))
            .width(Length::Fixed(48.0))
            .height(Length::Fixed(48.0))
            .style(move |_| container::Style {
                background: Some(pair.color.into()),
                ..Default::default()
            }),
        container(text(name).style(|theme: &cosmic::iced::Theme| text::Style {
            color: Some(theme.palette().text),
            ..Default::default()
        }))
        .style(|theme: &cosmic::iced::Theme| container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            border: Border {
                color: theme.extended_palette().background.base.color,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .padding(8),
        tooltip::Position::Top,
    )
    .into()
}

pub fn main() -> cosmic::iced::Result {
    cosmic::iced::application(State::default, State::update, State::view)
        .theme(State::theme)
        .title("Animated Theme")
        .run()
}
