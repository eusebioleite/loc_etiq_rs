#![windows_subsystem = "windows"]

mod core;
mod ui;
mod update;
mod config;

use iced::{ window, Theme };

use crate::core::state::State;

pub fn main() -> iced::Result {
    match config::init() {
        Ok(c) => c,
        Err(_) => panic!("Não encontrado arquivo de configuração.")
    };

    // Parâmetros da janela
    let window_settings = window::Settings {
        size: iced::Size::new(520.0, 580.0),
        resizable: false,
        decorations: false,
        exit_on_close_request: true,
        closeable: true,
        ..Default::default()
    };

    // Inicialização da aplicação
    iced::application(|| State::default(), update::handle, ui::view)
        .title(|_state: &State| "Locais de Estoque".to_string())
        .window(window_settings)
        .theme(|_state: &State| Theme::CatppuccinLatte)
        .centered()
        .run()
}
