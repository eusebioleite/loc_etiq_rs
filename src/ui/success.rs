use iced::widget::{container, text};
use iced::{Alignment, Color, Element, Length};

use crate::core::message::Message;

pub fn render<'a>() -> Element<'a, Message> {
    container(text("QR Code copiado para área de transferência!").size(12))
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(2)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgb(0.1, 0.7, 0.3).into()),
            text_color: Some(Color::WHITE),
            border: iced::Border {
                radius: (0.0).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}
