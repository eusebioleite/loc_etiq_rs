pub mod button;
pub mod content;
pub mod dropdown;
pub mod qtd_copias;
pub mod searchbar;
pub mod styles;
pub mod table;
pub mod toast;
pub mod toolbar;

use iced::widget::{column, container, stack, Space};
use iced::{Alignment, Element, Length};

use crate::core::message::{Message, Toast};
use crate::core::state::State;

pub fn view(state: &State) -> Element<'_, Message> {
    let toolbar = toolbar::render();
    let content = content::render(&state);
    let success = toast::render(&state, Toast::Success);
    let error = toast::render(&state, Toast::Error);

    let mut main_stack = stack![content];
    if state.show_success {
        let toast = container(success)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::End);

        main_stack = main_stack.push(toast);
    } else if state.show_error {
        let toast = container(error)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::End);

        main_stack = main_stack.push(toast);
    }

    column![toolbar, Space::new().height(10), main_stack].into()
}
