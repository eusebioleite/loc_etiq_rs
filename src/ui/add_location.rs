use iced::widget::{button, container, row, text, text_input};
use iced::{Alignment, Element, Length};
use lucide_icons::iced::icon_plus;

use crate::core::message::Message;
use crate::core::state::State;
use crate::ui::styles;

pub fn render<'a>(state: &'a State) -> Element<'a, Message> {
    container(
        row![
            text("Novo Local:")
                .size(16)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(styles::TEXT_DARK)
                }),
            text_input("Ex: PRAT-01...", &state.new_location_input)
                .on_input(Message::NewLocationInputChanged)
                .on_submit(Message::AddLocation)
                .size(16),
            button(icon_plus().size(16))
                .on_press(Message::AddLocation)
                .style(styles::primary_button)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(5)
    .into()
}
