#![windows_subsystem = "windows"]

mod config;
mod core;
mod ui;
mod update;

use crate::core::state::State;
use iced::{Theme, window};
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn main() -> iced::Result {
    match config::init() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };

    let window_settings = window::Settings {
        size: iced::Size::new(520.0, 580.0),
        resizable: false,
        decorations: false,
        exit_on_close_request: true,
        closeable: true,
        ..Default::default()
    };

    iced::application(|| State::default(), update::handle, ui::view)
        .title(|_state: &State| "Locais de Estoque".to_string())
        .window(window_settings)
        .theme(|_state: &State| Theme::Light)
        .font(LUCIDE_FONT_BYTES)
        .centered()
        .run()
}
