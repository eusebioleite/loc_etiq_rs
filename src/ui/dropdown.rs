use iced::widget::{container, pick_list};
use iced::{Alignment, Element, Length, Padding};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    let printer_select = pick_list(&state.printers[..], state.selected_printer.as_ref(), |_p| {
        Message::ClearNotification
    })
    .placeholder("Impressora...")
    .text_size(12)
    .width(Length::Fill);

    container(printer_select)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(Padding {
            bottom: 10.0,
            top: 0.0,
            right: 5.0,
            left: 5.0,
        })
        .into()
}
