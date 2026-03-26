use crate::core::message::Message;
use crate::core::printer::print;
use crate::core::state::{ State };
use iced::window;

pub fn handle(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        // Connections
        Message::Connections => {
            state.msg_toast = "Recurso em desenvolvimento.".to_string();
            state.show_error = true;
            iced::Task::perform(
                async {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                },
                |_| Message::ClearNotification
            )
        }
        // Print
        Message::Print => {
            let printer = state.selected_printer.clone().unwrap_or_else(|| "Empty".to_string());
            let amount_copies = state.count_copies.clone();
            let locations: Vec<String> = state.locations
                .clone()
                .into_iter()
                .filter(|local| { local.selected })
                .map(|local| local.description)
                .collect();

            if printer == "Empty" {
                state.msg_toast = "Nenhuma impressora selecionada.".to_string();
                state.show_error = true;
                iced::Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    },
                    |_| Message::ClearNotification
                )
            } else if locations.len() == 0 {
                state.msg_toast = "Nenhum local selecionado.".to_string();
                state.show_error = true;
                iced::Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    },
                    |_| Message::ClearNotification
                )
            } else {
                for location in locations {
                    for _ in 1..=amount_copies {
                        match print(printer.as_str(), location.as_str()) {
                            Ok(_) => {
                                state.msg_toast = "Etiqueta enviada para impressora.".to_string();
                                state.show_success = true;
                            }
                            Err(_) => {
                                state.msg_toast = "Erro ao imprimir etiqueta.".to_string();
                                state.show_error = true;
                            }
                        }
                    }
                }
                iced::Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    },
                    |_| Message::ClearNotification
                )
            }
        }

        // Dropdown Selected
        Message::PrinterSelected(printer) => {
            state.selected_printer = Some(printer);
            iced::Task::none()
        }

        // Copies Slider
        Message::CopiesChanged(count_copies) => {
            state.count_copies = count_copies;
            iced::Task::none()
        }

        // Search
        Message::SearchContents(search_query) => {
            state.search_query = search_query;
            iced::Task::none()
        }

        // Selection
        Message::ToggleLocation(location, selected) => {
            if let Some(row) = state.locations.iter_mut().find(|loc| loc.description == location) {
                row.selected = selected;
            }
            iced::Task::none()
        }

        // Notification
        Message::ClearNotification => {
            state.show_success = false;
            state.show_error = false;
            iced::Task::none()
        }

        // Default
        Message::WindowDrag => window::oldest().and_then(|id| window::drag(id)),
        Message::CloseRequested => iced::exit(),
        Message::MinimizeRequested => window::oldest().and_then(|id| window::minimize(id, true)),
    }
}
