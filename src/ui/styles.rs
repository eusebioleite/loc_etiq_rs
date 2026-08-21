use iced::widget::{button, container, slider};
use iced::{Color, Theme};

// Cores base extraídas do seu layout

pub const BG: Color = Color::from_rgb(5.0 / 255.0, 124.0 / 255.0, 105.0 / 255.0);
pub const SUCCESS: Color = Color::from_rgb(40.0 / 255.0, 167.0 / 255.0, 69.0 / 255.0);
pub const ERROR: Color = Color::from_rgb(220.0 / 255.0, 53.0 / 255.0, 69.0 / 255.0);

pub fn error_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(ERROR.into()),
        text_color: Some(Color::WHITE),
        border: iced::Border {
            radius: (0.0).into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn success_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(SUCCESS.into()),
        text_color: Some(Color::WHITE),
        border: iced::Border {
            radius: (0.0).into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn toolbar_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(BG.into()),
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

pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base_color = Color::from_rgb8(5, 124, 105);
    let hover_color = Color::from_rgb8(7, 144, 120);
    let pressed_color = Color::from_rgb8(3, 100, 85);

    button::Style {
        background: match status {
            button::Status::Hovered => Some(hover_color.into()),
            button::Status::Pressed => Some(pressed_color.into()),
            _ => Some(base_color.into()),
        },
        text_color: Color::WHITE,
        border: iced::Border {
            radius: (4.0).into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn slider_style(_theme: &Theme, status: slider::Status) -> slider::Style {
    let base_color = Color::from_rgb8(5, 124, 105);
    let hover_color = Color::from_rgb8(7, 144, 120);
    let drag_color = Color::from_rgb8(3, 100, 85);

    let color = match status {
        slider::Status::Active => base_color,
        slider::Status::Hovered => hover_color,
        slider::Status::Dragged => drag_color,
    };

    slider::Style {
        rail: slider::Rail {
            backgrounds: (color.into(), Color::WHITE.into()),
            width: 4.0,
            border: iced::Border {
                radius: (2.0).into(),
                width: 0.0,
                ..Default::default()
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 7.0 },
            background: color.into(),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
    }
}
