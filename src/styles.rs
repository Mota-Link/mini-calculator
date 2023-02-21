use super::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, text, Button};
use iced::{color, theme, Background, Length, Theme};

pub const BUTTON_DIAMETER: u16 = 66;
pub const BUTTON_SPACE: u16 = 5;

pub fn create_button(
    content: &str,
    msg: Message,
    style: impl button::StyleSheet<Style = Theme> + 'static,
) -> Button<Message> {
    let content = text(content)
        .size(27)
        .width(Length::Fill)
        .height(Length::Fill)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center);

    button(content)
        .width(BUTTON_DIAMETER)
        .height(BUTTON_DIAMETER)
        .style(theme::Button::Custom(Box::new(style)))
        .on_press(msg)
}

pub struct NumStyle;
impl button::StyleSheet for NumStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            // shadow_offset: iced::Vector { x: 0., y: 3. },
            border_radius: (BUTTON_DIAMETER / 2) as f32,
            background: Some(Background::Color(color!(0xF5E9CF))),
            text_color: color!(0xE96479),
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(color!(0xF5E9CF, 0.8))),
            ..Self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(color!(0x7DB9B6))),
            text_color: color!(0xF5E9CF),
            ..Self.active(style)
        }
    }
}

pub struct OpsStyle;
impl button::StyleSheet for OpsStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: (BUTTON_DIAMETER / 2) as f32,
            background: Some(Background::Color(color!(0xE96479))),
            text_color: color!(0xF5E9CF),
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(color!(0xE96479, 0.5))),
            ..Self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(color!(0x7DB9B6))),
            ..Self.active(style)
        }
    }
}

pub fn screen_container_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(color!(0xE96479)),
        background: Some(Background::Color(color!(0xF5E9CF))),
        border_radius: (BUTTON_DIAMETER / 2) as f32,
        ..Default::default()
    }
}

pub fn calculator_background_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(color!(0x4D455D))),
        ..Default::default()
    }
}
