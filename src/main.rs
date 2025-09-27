#![windows_subsystem = "windows"]

pub mod styles;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, container, row, text};
use iced::window::{icon, Settings};
use iced::{Element, Length, Size};
use image::ImageFormat;
use std::f64::{EPSILON, NAN};

use styles::*;
use NumMessage::*;
use OpsMessage::*;

fn main() -> iced::Result {
    let my_icon = include_bytes!("../icon.png");
    iced::application("Mini Calculator", update, view)
        .window(Settings {
            resizable: false,
            size: Size::new(300f32, 500f32),
            icon: Some(icon::from_file_data(my_icon, Some(ImageFormat::Png)).unwrap()),
            ..Settings::default()
        })
        .run()
}

pub struct State {
    display: String,
    history: String,
    num_buf: [f64; 2],
    ops_buf: char,
}

impl Default for State {
    fn default() -> Self {
        Self {
            display: "0".to_owned(),
            history: "".to_owned(),
            num_buf: [NAN, NAN],
            ops_buf: '\0',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Num(NumMessage),
    Ops(OpsMessage),
    Enter,
    CE,
    C,
    DEL,
    Style,
}

#[derive(Debug, Clone, Copy)]
pub enum NumMessage {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Dot,
}

#[derive(Debug, Clone, Copy)]
pub enum OpsMessage {
    Add,
    Sub,
    Mul,
    Div,
}

fn update(state: &mut State, message: Message) {
    let format = |input: f64| -> String {
        if input.to_string().len() >= 10 {
            format!("{:.3e}", input)
        } else {
            input.to_string()
        }
    };

    match message {
        Message::Num(num) if (num as u8) < 10 => {
            if !state.num_buf[1].is_nan() {
                state.num_buf[1] = NAN;
                state.ops_buf = '\0';
                state.history.clear();
                state.display.clear();
            } else if state.display == "0" {
                state.display.clear();
            }

            state.display.push_str(&(num as u8).to_string());
        }
        Message::Num(_) => {
            if !state.num_buf[1].is_nan() {
                state.num_buf[1] = NAN;
                state.ops_buf = '\0';
                state.display = "0".to_owned();
            } else if state.display.contains(".") {
                return;
            }

            state.display.push('.');
        }
        Message::Ops(ops) => {
            let ops = match ops {
                OpsMessage::Add => '+',
                OpsMessage::Sub => '-',
                OpsMessage::Mul => '*',
                OpsMessage::Div => '/',
            };

            match state.num_buf {
                [a, b] if a.is_nan() && b.is_nan() => {
                    state.num_buf[0] = state.display.parse().unwrap();
                    state.history = format!("{} {}", format(state.display.parse().unwrap()), ops);
                    state.display = "0".to_owned();
                    state.ops_buf = ops;
                }
                [a, _] if a.is_nan() => {
                    state.num_buf[0] = state.display.parse().unwrap();
                    state.num_buf[1] = NAN;
                    state.history = format!("{} {}", format(state.display.parse().unwrap()), ops);
                    state.display = "0".to_owned();
                    state.ops_buf = ops;
                }
                [_, b] if b.is_nan() => {
                    let num: f64 = state.display.parse().unwrap();
                    if (num - 0.).abs() < EPSILON {
                        state.ops_buf = ops;
                        return;
                    }

                    let answer = match state.ops_buf {
                        '+' => state.num_buf[0] + num,
                        '-' => state.num_buf[0] - num,
                        '*' => state.num_buf[0] * num,
                        '/' => state.num_buf[0] / num,
                        _ => unreachable!(),
                    };

                    state.history = format!("{} {}", format(answer), ops);
                    state.display = "0".to_owned();
                    state.num_buf[0] = answer;
                    state.ops_buf = ops;
                }
                [_, _] => unreachable!(),
            }
        }
        Message::Enter => match state.num_buf {
            [a, b] if a.is_nan() && b.is_nan() => {
                state.history = format!("{} =", format(state.display.parse().unwrap()));
                state.num_buf[1] = state.display.parse().unwrap();
            }
            [_, b] if b.is_nan() => {
                let num: f64 = state.display.parse().unwrap();
                let answer = match state.ops_buf {
                    '+' => state.num_buf[0] + num,
                    '-' => state.num_buf[0] - num,
                    '*' => state.num_buf[0] * num,
                    '/' => state.num_buf[0] / num,
                    _ => unreachable!(),
                };
                state.display = format(answer);
                state.history = format!(
                    "{} {} {} =",
                    format(state.num_buf[0]),
                    state.ops_buf,
                    format(num)
                );
                state.num_buf = [NAN, num];
            }
            [a, _] if a.is_nan() => {
                if state.ops_buf == '\0' {
                    state.history = format!("{} =", format(state.display.parse().unwrap()));
                    state.num_buf[1] = state.display.parse().unwrap();
                } else {
                    let num: f64 = state.display.parse().unwrap();
                    let answer = match state.ops_buf {
                        '+' => num + state.num_buf[1],
                        '-' => num - state.num_buf[1],
                        '*' => num * state.num_buf[1],
                        '/' => num / state.num_buf[1],
                        _ => unreachable!(),
                    };

                    state.display = format(answer);
                    state.history = format!(
                        "{} {} {} =",
                        format(num),
                        state.ops_buf,
                        format(state.num_buf[1])
                    );
                }
            }
            [_, _] => unreachable!(),
        },
        Message::DEL => match state.num_buf {
            // [a, b] if a.is_nan() && b.is_nan() => todo!(),
            [_, b] if b.is_nan() => {
                state.display.pop();
                if state.display.len() == 0 {
                    state.display = "0".to_owned();
                }
            }
            [a, _] if a.is_nan() => state.history = "".to_owned(),
            [_, _] => unreachable!(),
        },
        Message::C => *state = State::default(),
        Message::CE => match state.num_buf {
            [_, b] if b.is_nan() => state.display = "0".to_owned(),
            [a, _] if a.is_nan() => *state = State::default(),
            [_, _] => unreachable!(),
        },
        Message::Style => unsafe { STYLE_IDX = (STYLE_IDX + 1) % 3 },
    };
}

fn view(state: &State) -> Element<'_, Message> {
    let keyboard = row![
        column![
            create_button("STY", Message::Style, enter_button_style),
            create_button("+", Message::Ops(Add), operator_button_style),
            create_button("-", Message::Ops(Sub), operator_button_style),
            create_button("*", Message::Ops(Mul), operator_button_style),
            create_button("/", Message::Ops(Div), operator_button_style),
        ]
        .spacing(BUTTON_SPACE)
        .align_x(Horizontal::Center),
        column![
            create_button("CE", Message::CE, enter_button_style),
            create_button("7", Message::Num(Seven), number_button_style),
            create_button("4", Message::Num(Four), number_button_style),
            create_button("1", Message::Num(One), number_button_style),
            create_button(".", Message::Num(Dot), operator_button_style),
        ]
        .spacing(BUTTON_SPACE),
        column![
            create_button("C", Message::C, enter_button_style),
            create_button("8", Message::Num(Eight), number_button_style),
            create_button("5", Message::Num(Five), number_button_style),
            create_button("2", Message::Num(Two), number_button_style),
            create_button("0", Message::Num(Zero), number_button_style),
        ]
        .spacing(BUTTON_SPACE),
        column![
            create_button("DEL", Message::DEL, enter_button_style),
            create_button("9", Message::Num(Nine), number_button_style),
            create_button("6", Message::Num(Six), number_button_style),
            create_button("3", Message::Num(Three), number_button_style),
            create_button("=", Message::Enter, enter_button_style),
        ]
        .spacing(BUTTON_SPACE),
    ]
    .spacing(BUTTON_SPACE)
    .align_y(Vertical::Center);

    let screen = container(
        column![text(&state.history).size(22), text(&state.display).size(50)]
            .spacing(5)
            .align_x(Horizontal::Right),
    )
    .width(279)
    .height(120)
    .center_y(Length::Fill)
    .align_x(Horizontal::Right)
    .padding(20)
    .style(screen_container_style);

    container(column![screen, keyboard].spacing(10))
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .style(calculator_background_style)
        .into()
}
