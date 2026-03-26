use iced::widget::{container, text};
use iced::{Alignment, Element, Length};

use crate::core::message::Message;
use crate::core::message::Toast;
use crate::core::state::State;
use crate::ui::styles;
pub fn render<'a>(state: &'a State, toast: Toast) -> Element<'a, Message> {
    match toast {
        Toast::Success => container(text(&state.msg_toast).size(12))
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(2)
            .style(styles::success_container)
            .into(),
        Toast::Error => container(text(&state.msg_toast).size(12))
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(2)
            .style(styles::error_container)
            .into(),
    }
}
