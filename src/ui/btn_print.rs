use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Element, Length, Padding};

use crate::core::message::Message;
use crate::ui::styles;

pub fn render<'a>() -> Element<'a, Message> {
    container(
        button(row![
            Space::new().width(Length::Fill),
            text("Imprimir").size(24),
            Space::new().width(Length::Fill)
        ])
        .on_press(Message::Print)
        .style(styles::primary_button)
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
