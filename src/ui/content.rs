use iced::widget::{container, Space};
use iced::{Element, Length};

use crate::core::message::Message;

pub fn render<'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
    .into()
}
