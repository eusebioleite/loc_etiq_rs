use iced::widget::{button, container};
use iced::{Color, Theme};

// Cores base extraídas do seu layout
pub const BG_BLUE: Color = Color::from_rgb(1.0 / 255.0, 120.0 / 255.0, 255.0 / 255.0); // Alternativa otimizada para from_rgb8(1, 120, 255)

pub fn toolbar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG_BLUE.into()),
        ..Default::default()
    }
}

pub fn minimize_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base_color = Color::from_rgb8(233, 146, 9);
    let hover_color = Color::from_rgb8(243, 156, 18);
    let pressed_color = Color::from_rgb8(243, 156, 18);

    button::Style {
        background: match status {
            button::Status::Hovered => Some(hover_color.into()),
            button::Status::Pressed => Some(pressed_color.into()),
            _ => Some(base_color.into()),
        },
        text_color: Color::WHITE,
        border: iced::Border {
            radius: (0.0).into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn close_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base_color = Color::from_rgb8(231, 76, 60);
    let hover_color = Color::from_rgb8(255, 100, 100);
    let pressed_color = Color::from_rgb8(255, 100, 100);

    button::Style {
        background: match status {
            button::Status::Hovered => Some(hover_color.into()),
            button::Status::Pressed => Some(pressed_color.into()),
            _ => Some(base_color.into()),
        },
        text_color: Color::WHITE,
        border: iced::Border {
            radius: (0.0).into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
