use iced::widget::{button, container, row, text, Space};
use iced::{Alignment, Element, Length, Padding};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    container(
        button(row![
            Space::new().width(Length::Fill),
            text("Gerar Etiquetas").size(12),
            Space::new().width(Length::Fill)
        ])
        .on_press(Message::Print)
        .width(Length::Fill),
    )
    .align_x(Alignment::Center)
    .center_x(Length::Fill)
    .padding(Padding {
        bottom: 5.0,
        top: 0.0,
        right: 5.0,
        left: 5.0,
    })
    .into()
}
