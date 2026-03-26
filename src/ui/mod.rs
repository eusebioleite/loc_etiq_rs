pub mod button;
pub mod content;
pub mod dropdown;
pub mod qtd_copias;
pub mod searchbar;
pub mod styles;
pub mod success;
pub mod table;
pub mod toolbar;

use iced::widget::{column, stack, Space};
use iced::Element;

use crate::core::message::Message;
use crate::core::state::State;

pub fn view(state: &State) -> Element<'_, Message> {
    let toolbar = toolbar::render();
    let content = content::render(&state);
    let success = success::render();

    if state.show_success {
        column![
            toolbar,
            Space::new().height(10),
            content,
            stack![success, Space::new().height(10)]
        ]
        .into()
    } else {
        column![toolbar, Space::new().height(10), content].into()
    }
}
