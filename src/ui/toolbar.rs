use iced::widget::{button, container, row, text, MouseArea, Space, image};
use iced::{Element, Length};

use crate::core::message::Message;
use crate::ui::styles;

pub fn render<'a>() -> Element<'a, Message> {
    MouseArea::new(
        container(
            row![
                image(image::Handle::from_path("assets/logo.png")).width(24).height(24),
                Space::new().width(Length::Fill),
                button(text("—").size(16))
                    .style(styles::minimize_button)
                    .padding(5)
                    .height(Length::Fill)
                    .on_press(Message::MinimizeRequested),
                button(text("✕").size(16))
                    .style(styles::close_button)
                    .padding(5)
                    .height(Length::Fill)
                    .on_press(Message::CloseRequested)
            ]
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(30)
        .style(styles::toolbar_container),
    )
    .on_press(Message::WindowDrag)
    .into()
}
