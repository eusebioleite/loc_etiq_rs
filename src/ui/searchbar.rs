use iced::widget::{container, text_input};
use iced::{Alignment, Element, Length};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    container(
        text_input("Pesquisar..", &state.search_query)
            .on_input(Message::SearchContents)
            .size(18),
    )
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(5)
    .into()
}
