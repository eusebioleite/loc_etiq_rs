mod core;
mod ui;
mod update;

use iced::{window, Theme};

use crate::core::state::State;

pub fn main() -> iced::Result {

    // Parâmetros da janela
    let window_settings = window::Settings {
        size: iced::Size::new(320.0, 180.0),
        resizable: false,
        decorations: false,
        exit_on_close_request: true,
        closeable: true,
        ..Default::default()
    };

    // Inicialização da aplicação
    iced::application(|| State::default(), update::handle, ui::view)
        .title(|_state: &State| "Template".to_string())
        .window(window_settings)
        .theme(|_state: &State| Theme::CatppuccinLatte)
        .centered()
        .run()
}