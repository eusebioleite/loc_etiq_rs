use iced::widget::{column, container, row};
use iced::{Alignment, Element, Length};

use crate::core::message::Message;
use crate::core::state::State;
use crate::ui::add_location;
use crate::ui::btn_print;
use crate::ui::dropdown;
use crate::ui::qtd_copias;
use crate::ui::searchbar;
use crate::ui::table;
pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let searchbar = searchbar::render(&state);
    let add_location = add_location::render(&state);
    let btn_print = btn_print::render();
    let table = table::render(&state);
    let dpd = dropdown::render(&state);
    let slider = qtd_copias::render(&state);
    container(column![searchbar, add_location, table, row![dpd, slider], btn_print])
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .align_x(Alignment::Center)
        .into()
}
