use super::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, text, Button};
use iced::{color, theme, Background, Color, Length, Theme};
use lazy_static::lazy_static;

pub const BUTTON_DIAMETER: u16 = 66;
pub const BUTTON_SPACE: u16 = 5;

struct ColorStyles {
    background: Color,
    text: Color,
    screen_background: Color,
    screen_text: Color,
    num_button_active: Color,
    num_button_hover: Color,
    num_button_press: Color,
    ops_button_active: Color,
    ops_button_hover: Color,
    ops_button_press: Color,
    etr_button_active: Color,
    etr_button_hover: Color,
    etr_button_press: Color,
}

fn create_color_style(
    background: u32,
    text: u32,
    screen_background: u32,
    screen_text: u32,
    num_button_active: u32,
    num_button_hover: u32,
    num_button_press: u32,
    ops_button_active: u32,
    ops_button_hover: u32,
    ops_button_press: u32,
    etr_button_active: u32,
    etr_button_hover: u32,
    etr_button_press: u32,
) -> ColorStyles {
    ColorStyles {
        background: color!(background),
        text: color!(text),
        screen_text: color!(screen_text),
        screen_background: color!(screen_background),
        num_button_active: color!(num_button_active),
        num_button_hover: color!(num_button_hover),
        num_button_press: color!(num_button_press),
        ops_button_active: color!(ops_button_active),
        ops_button_hover: color!(ops_button_hover),
        ops_button_press: color!(ops_button_press),
        etr_button_active: color!(etr_button_active),
        etr_button_hover: color!(etr_button_hover),
        etr_button_press: color!(etr_button_press),
    }
}

lazy_static! {
    static ref COLORS: [ColorStyles; 3] = [
        create_color_style(
            0x142F4D, 0xFAEAD3, 0xFAEAD3, 0xDA674C, 0xDD8238, 0xE9A26A, 0xE18039, 0x2B5D68,
            0x76A6B0, 0x2B5D68, 0xDA674C, 0xDF7C65, 0xD54F36,
        ),
        create_color_style(
            0xF5ECE5, 0x6D4D43, 0xFFFFFF, 0xDEAC9E, 0xE8C6BC, 0xEFDCD6, 0xE0AC9E, 0x93B4AD,
            0xB5C6BE, 0x7FAB9E, 0xD98481, 0xE5B6AE, 0xCA6F68
        ),
        create_color_style(
            0xF7F4F1, 0xFFFFFF, 0xAFCBD7, 0xF6F5F1, 0xF2DCC5, 0xF0D3B5, 0xEEC49C, 0x99CED6,
            0x75BFCA, 0x4BA3AD, 0xE8A89F, 0xDD938A, 0xD87F74,
        ),
    ];
}

pub static mut STYLE_IDX: usize = 0;

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
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].num_button_active,
            )),
            text_color: COLORS[unsafe { STYLE_IDX }].text,
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].num_button_hover,
            )),
            ..Self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].num_button_press,
            )),
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
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].ops_button_active,
            )),
            text_color: COLORS[unsafe { STYLE_IDX }].text,
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].ops_button_hover,
            )),
            ..Self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].ops_button_press,
            )),
            ..Self.active(style)
        }
    }
}

pub struct EtrStyle;
impl button::StyleSheet for EtrStyle {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: (BUTTON_DIAMETER / 2) as f32,
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].etr_button_active,
            )),
            text_color: COLORS[unsafe { STYLE_IDX }].text,
            ..Default::default()
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].etr_button_hover,
            )),
            ..Self.active(style)
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(
                COLORS[unsafe { STYLE_IDX }].etr_button_press,
            )),
            ..Self.active(style)
        }
    }
}

pub fn screen_container_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(COLORS[unsafe { STYLE_IDX }].screen_text),
        background: Some(Background::Color(
            COLORS[unsafe { STYLE_IDX }].screen_background,
        )),
        border_radius: (BUTTON_DIAMETER / 2) as f32,
        ..Default::default()
    }
}

pub fn calculator_background_style(_: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(COLORS[unsafe { STYLE_IDX }].background)),
        ..Default::default()
    }
}
