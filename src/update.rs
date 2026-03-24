use iced::window;
use crate::core::message::Message;
use crate::core::state::State;

pub fn handle(_state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::WindowDrag => window::oldest().and_then(|id| window::drag(id)),
        Message::CloseRequested => iced::exit(),
        Message::MinimizeRequested => window::oldest().and_then(|id| window::minimize(id, true)),
    }
}