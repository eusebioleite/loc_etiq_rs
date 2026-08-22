use iced::widget::{MouseArea, Space, button, container, row, text};
use iced::{Element, Length};

use crate::core::message::Message;
use crate::ui::styles;

pub fn render<'a>() -> Element<'a, Message> {
    MouseArea::new(
        container(
            row![
                Space::new().width(Length::Fixed(20.0)),
                text("Imprimir Etiquetas de Local de Estoque")
                    .size(16)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(styles::TEXT_LIGHT)
                    }),
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
