use iced::widget::{checkbox, column, container, scrollable};
use iced::{Element, Length, Padding};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let filtered_rows = state.locations.iter().filter(|row| {
        state.search_query.is_empty()
            || row
                .description
                .to_lowercase()
                .contains(&state.search_query.to_lowercase())
    });

    let mut list_content = column![].spacing(10).padding(10);

    for row in filtered_rows {
        let row_val = row.clone();
        list_content = list_content.push(
            checkbox(row.selected)
                .label(&row.description)
                .on_toggle(move |is_checked| {
                    Message::ToggleLocation(row_val.description.clone(), is_checked)
                })
                .size(20)
                .width(Length::Fill),
        );
    }

    container(scrollable(list_content))
        .height(Length::Fixed(240.0))
        .width(Length::Fill)
        .padding(Padding {
            bottom: 10.0,
            top: 10.0,
            right: 10.0,
            left: 10.0,
        })
        .into()
}
