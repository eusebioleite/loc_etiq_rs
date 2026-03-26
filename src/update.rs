use crate::core::message::Message;
use crate::core::state::State;
use iced::window;

pub fn handle(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        // Print
        Message::Print => {
            state.msg_toast = "Etiqueta enviada para impressora.".to_string();
            state.show_success = true;
            iced::Task::perform(
                async {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                },
                |_| Message::ClearNotification
            )
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
