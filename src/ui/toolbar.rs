use iced::widget::{button, container, row, text, MouseArea, Space};
use iced::{Element, Length};

use crate::core::message::Message;
use crate::ui::styles;

pub fn render<'a>() -> Element<'a, Message> {
    MouseArea::new(
        container(row![
            Space::new().width(Length::Fill),
            button(text("—").size(12))
                .style(styles::minimize_button)
                .padding(3)
                .on_press(Message::MinimizeRequested),
            button(text("✕").size(12))
                .style(styles::close_button)
                .padding(3)
                .on_press(Message::CloseRequested)
        ])
        .width(Length::Fill)
        .height(20)
        .style(styles::toolbar_container),
    )
    .on_press(Message::WindowDrag)
    .into()
}
