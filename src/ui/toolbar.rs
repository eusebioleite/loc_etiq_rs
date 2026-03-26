use iced::widget::{button, container, row, text, MouseArea, Space};
use iced::{Element, Font, Length};

use crate::core::message::Message;
use crate::ui::styles;
const NERD_FONT_BYTES: &[u8] = include_bytes!("..\\..\\fonts\\SymbolsNerdFont.ttf");
const ICON_FONT: Font = Font::with_name("Symbols Nerd Font");

pub fn render<'a>() -> Element<'a, Message> {
    MouseArea::new(
        container(
            row![
                button(text("\u{f423}").font(ICON_FONT).size(18))
                    .on_press(Message::Connections)
                    .height(Length::Fill)
                    .width(Length::Shrink)
                    .style(styles::connection_button),
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
