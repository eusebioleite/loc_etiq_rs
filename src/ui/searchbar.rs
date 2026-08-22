use iced::widget::{checkbox, container, row, text_input};
use iced::{Alignment, Element, Length};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let query = state.search_query.to_lowercase();
    let is_empty = query.is_empty();

    let mut visible_count = 0;
    let mut selected_count = 0;

    for row in state.locations.iter() {
        if is_empty || row.description.to_lowercase().contains(&query) {
            visible_count += 1;
            if row.selected {
                selected_count += 1;
            }
        }
    }

    let all_selected = visible_count > 0 && visible_count == selected_count;

    container(
        row![
            text_input("Pesquisar..", &state.search_query)
                .on_input(Message::SearchContents)
                .size(18),
            checkbox(all_selected)
                .size(20)
                .on_toggle(Message::ToggleAllVisible)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(5)
    .into()
}
