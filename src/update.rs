use crate::core::message::Message;
use crate::core::printer::print;
use crate::core::state::State;
use iced::window;

pub fn handle(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        // Print
        Message::Print => {
            let printer = state
                .selected_printer
                .clone()
                .unwrap_or_else(|| "Empty".to_string());
            let amount_copies = state.count_copies.clone();
            let locations: Vec<String> = state
                .locations
                .clone()
                .into_iter()
                .filter(|local| local.selected)
                .map(|local| local.description)
                .collect();

            if printer == "Empty" {
                state.msg_toast = "Nenhuma impressora selecionada.".to_string();
                state.show_error = true;
                iced::Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    },
                    |_| Message::ClearNotification,
                )
            } else if locations.len() == 0 {
                state.msg_toast = "Nenhum local selecionado.".to_string();
                state.show_error = true;
                iced::Task::perform(
                    async {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    },
                    |_| Message::ClearNotification,
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
                    |_| Message::ClearNotification,
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
            if let Some(row) = state
                .locations
                .iter_mut()
                .find(|loc| loc.description == location)
            {
                row.selected = selected;
            }
            iced::Task::none()
        }

        // Toggle All Visible
        Message::ToggleAllVisible(selected) => {
            let query = state.search_query.to_lowercase();
            let is_empty = query.is_empty();
            for row in state.locations.iter_mut() {
                if is_empty || row.description.to_lowercase().contains(&query) {
                    row.selected = selected;
                }
            }
            iced::Task::none()
        }

        // Notification
        Message::ClearNotification => {
            state.show_success = false;
            state.show_error = false;
            iced::Task::none()
        }

        // Add Location
        Message::NewLocationInputChanged(input) => {
            state.new_location_input = input;
            iced::Task::none()
        }
        Message::AddLocation => {
            let input = state.new_location_input.trim().to_string();
            if !input.is_empty() {
                if !state.locations.iter().any(|loc| loc.description == input) {
                    state.locations.push(crate::core::state::TableRow {
                        description: input.clone(),
                        selected: false,
                    });

                    match crate::config::add_location(input) {
                        Ok(_) => {
                            state.msg_toast = "Local adicionado com sucesso.".to_string();
                            state.show_success = true;
                            state.new_location_input.clear();
                        }
                        Err(e) => {
                            state.msg_toast = format!("Erro ao salvar: {}", e);
                            state.show_error = true;
                        }
                    }

                    iced::Task::perform(
                        async {
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        },
                        |_| Message::ClearNotification,
                    )
                } else {
                    state.msg_toast = "Local já existe.".to_string();
                    state.show_error = true;
                    iced::Task::perform(
                        async {
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        },
                        |_| Message::ClearNotification,
                    )
                }
            } else {
                iced::Task::none()
            }
        }

        // Delete Location
        Message::DeleteLocation(location) => {
            state.locations.retain(|loc| loc.description != location);

            match crate::config::remove_location(&location) {
                Ok(_) => {
                    state.msg_toast = format!("Local '{}' removido com sucesso.", location);
                    state.show_success = true;
                }
                Err(e) => {
                    state.msg_toast = format!("Erro ao remover local: {}", e);
                    state.show_error = true;
                }
            }

            iced::Task::perform(
                async {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                },
                |_| Message::ClearNotification,
            )
        }

        // Default
        Message::WindowDrag => window::oldest().and_then(|id| window::drag(id)),
        Message::CloseRequested => iced::exit(),
        Message::MinimizeRequested => window::oldest().and_then(|id| window::minimize(id, true)),
    }
}
