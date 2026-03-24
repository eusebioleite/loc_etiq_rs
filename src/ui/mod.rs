pub mod styles;
pub mod toolbar;
pub mod content;

use iced::widget::{column, Space};
use iced::{Element};

use crate::core::message::Message;
use crate::core::state::State;

pub fn view(_state: &State) -> Element<'_, Message> {
    let toolbar = toolbar::render();
    let content = content::render();
    column![toolbar, Space::new().height(10), content].into()
}
