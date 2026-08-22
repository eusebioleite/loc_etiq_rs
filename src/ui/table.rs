use iced::widget::{button, checkbox, column, container, row, scrollable};
use iced::{Alignment, Element, Length, Padding};
use lucide_icons::iced::icon_x;

use crate::core::message::Message;
use crate::core::state::State;
use crate::ui::styles;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let filtered_rows = state.locations.iter().filter(|row| {
        state.search_query.is_empty()
            || row
                .description
                .to_lowercase()
                .contains(&state.search_query.to_lowercase())
    });

    let mut list_content = column![].spacing(10).padding(10);

    for row_item in filtered_rows {
        let desc = row_item.description.clone();
        let desc_del = desc.clone();

        list_content = list_content.push(
            row![
                checkbox(row_item.selected)
                    .label(&row_item.description)
                    .text_size(24)
                    .on_toggle(move |is_checked| {
                        Message::ToggleLocation(desc.clone(), is_checked)
                    })
                    .size(20)
                    .width(Length::Fill),
                button(icon_x().size(14))
                    .on_press(Message::DeleteLocation(desc_del))
                    .style(styles::delete_button)
                    .padding(Padding {
                        top: 4.0,
                        bottom: 4.0,
                        left: 8.0,
                        right: 8.0,
                    })
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        );
    }

    container(scrollable(list_content))
        .height(Length::Fixed(350.0))
        .width(Length::Fill)
        .padding(Padding {
            bottom: 10.0,
            top: 10.0,
            right: 10.0,
            left: 10.0,
        })
        .into()
}
