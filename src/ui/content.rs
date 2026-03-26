use iced::widget::{column, container, row};
use iced::{Alignment, Element, Length};

use crate::core::message::Message;
use crate::core::state::State;
use crate::ui::dropdown;
use crate::ui::qtd_copias;
use crate::ui::searchbar;
use crate::ui::table;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let searchbar = searchbar::render(&state);
    let btn = crate::ui::button::render(&state);
    let table = table::render(&state);
    let dpd = dropdown::render(&state);
    let slider = qtd_copias::render(&state);
    container(column![searchbar, table, row![dpd, slider], btn])
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .align_x(Alignment::Center)
        .into()
}
