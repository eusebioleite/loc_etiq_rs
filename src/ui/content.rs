use iced::widget::{button, column, container, text_input};
use iced::{Element, Length};

use crate::core::message::Message;
use crate::core::state::State;
use crate::ui::searchbar;
use crate::ui::table;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let searchbar = searchbar::render(&state);
    let btn = crate::ui::button::render(&state);
    let table = table::render(&state);
    container(column![searchbar, table, btn])
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}
