use iced::widget::{button, container, text_input};
use iced::{Alignment, Element, Length, Padding};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    container(button("Gerar Etiquetas").on_press(Message::ClearNotification))
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(Padding {
            bottom: 5.0,
            top: 0.0,
            right: 5.0,
            left: 5.0,
        })
        .into()
}
