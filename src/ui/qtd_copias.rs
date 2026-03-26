use iced::widget::{container, row, slider, text};
use iced::{Alignment, Element, Length, Padding};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    container(
        row![
            text("Cópias:").size(12),
            slider(1..=100, state.count_copies, Message::CopiesChanged).step(1),
            text(format!("{:02}", state.count_copies)).size(12)
        ]
        .spacing(5),
    )
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
