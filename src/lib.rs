pub mod styles;

use iced::alignment::Horizontal;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Sandbox, Theme};
use std::f64::{EPSILON, NAN};
use styles::*;
use NumMessage::*;
use OpsMessage::*;

pub struct Calculator {
    display: String,
    history: String,
    num_buf: [f64; 2],
    ops_buf: char,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Num(NumMessage),
    Ops(OpsMessage),
    Enter,
    CE,
    C,
    DEL,
    STYLE,
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

impl Sandbox for Calculator {
    type Message = Message;

    // Required methods
    fn new() -> Self {
        Self {
            display: "0".to_owned(),
            history: "".to_owned(),
            num_buf: [NAN, NAN],
            ops_buf: '\0',
        }
    }

    fn title(&self) -> String {
        "Mini Calculator".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        let format = |input: f64| -> String {
            if input.to_string().len() >= 10 {
                format!("{:.3e}", input)
            } else {
                input.to_string()
            }
        };

        match message {
            Message::Num(num) if (num as u8) < 10 => {
                if !self.num_buf[1].is_nan() {
                    self.num_buf[1] = NAN;
                    self.ops_buf = '\0';
                    self.history.clear();
                    self.display.clear();
                } else if self.display == "0" {
                    self.display.clear();
                }

                self.display.push_str(&(num as u8).to_string());
            }
            Message::Num(_) => {
                if !self.num_buf[1].is_nan() {
                    self.num_buf[1] = NAN;
                    self.ops_buf = '\0';
                    self.display = "0".to_owned();
                } else if self.display.contains(".") {
                    return;
                }

                self.display.push('.');
            }
            Message::Ops(ops) => {
                let ops = match ops {
                    OpsMessage::Add => '+',
                    OpsMessage::Sub => '-',
                    OpsMessage::Mul => '*',
                    OpsMessage::Div => '/',
                };

                match self.num_buf {
                    [a, b] if a.is_nan() && b.is_nan() => {
                        self.num_buf[0] = self.display.parse().unwrap();
                        self.history = format!("{} {}", format(self.display.parse().unwrap()), ops);
                        self.display = "0".to_owned();
                        self.ops_buf = ops;
                    }
                    [a, _] if a.is_nan() => {
                        self.num_buf[0] = self.display.parse().unwrap();
                        self.num_buf[1] = NAN;
                        self.history = format!("{} {}", format(self.display.parse().unwrap()), ops);
                        self.display = "0".to_owned();
                        self.ops_buf = ops;
                    }
                    [_, b] if b.is_nan() => {
                        let num: f64 = self.display.parse().unwrap();
                        if (num - 0.).abs() < EPSILON {
                            self.ops_buf = ops;
                            return;
                        }

                        let answer = match self.ops_buf {
                            '+' => self.num_buf[0] + num,
                            '-' => self.num_buf[0] - num,
                            '*' => self.num_buf[0] * num,
                            '/' => self.num_buf[0] / num,
                            _ => unreachable!(),
                        };

                        self.history = format!("{} {}", format(answer), ops);
                        self.display = "0".to_owned();
                        self.num_buf[0] = answer;
                        self.ops_buf = ops;
                    }
                    [_, _] => unreachable!(),
                }
            }
            Message::Enter => match self.num_buf {
                [a, b] if a.is_nan() && b.is_nan() => {
                    self.history = format!("{} =", format(self.display.parse().unwrap()));
                    self.num_buf[1] = self.display.parse().unwrap();
                }
                [_, b] if b.is_nan() => {
                    let num: f64 = self.display.parse().unwrap();
                    let answer = match self.ops_buf {
                        '+' => self.num_buf[0] + num,
                        '-' => self.num_buf[0] - num,
                        '*' => self.num_buf[0] * num,
                        '/' => self.num_buf[0] / num,
                        _ => unreachable!(),
                    };
                    self.display = format(answer);
                    self.history = format!(
                        "{} {} {} =",
                        format(self.num_buf[0]),
                        self.ops_buf,
                        format(num)
                    );
                    self.num_buf = [NAN, num];
                }
                [a, _] if a.is_nan() => {
                    if self.ops_buf == '\0' {
                        self.history = format!("{} =", format(self.display.parse().unwrap()));
                        self.num_buf[1] = self.display.parse().unwrap();
                    } else {
                        let num: f64 = self.display.parse().unwrap();
                        let answer = match self.ops_buf {
                            '+' => num + self.num_buf[1],
                            '-' => num - self.num_buf[1],
                            '*' => num * self.num_buf[1],
                            '/' => num / self.num_buf[1],
                            _ => unreachable!(),
                        };

                        self.display = format(answer);
                        self.history = format!(
                            "{} {} {} =",
                            format(num),
                            self.ops_buf,
                            format(self.num_buf[1])
                        );
                    }
                }
                [_, _] => unreachable!(),
            },
            Message::DEL => match self.num_buf {
                // [a, b] if a.is_nan() && b.is_nan() => todo!(),
                [_, b] if b.is_nan() => {
                    self.display.pop();
                    if self.display.len() == 0 {
                        self.display = "0".to_owned();
                    }
                }
                [a, _] if a.is_nan() => self.history = "".to_owned(),
                [_, _] => unreachable!(),
            },
            Message::C => *self = Self::new(),
            Message::CE => match self.num_buf {
                [_, b] if b.is_nan() => self.display = "0".to_owned(),
                [a, _] if a.is_nan() => *self = Self::new(),
                [_, _] => unreachable!(),
            },
            Message::STYLE => unsafe { STYLE_IDX = (STYLE_IDX + 1) % 3 },
        };
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let keyboard = row![
            column![
                create_button("STY", Message::STYLE, EtrStyle),
                create_button("+", Message::Ops(Add), OpsStyle),
                create_button("-", Message::Ops(Sub), OpsStyle),
                create_button("*", Message::Ops(Mul), OpsStyle),
                create_button("/", Message::Ops(Div), OpsStyle),
            ]
            .spacing(BUTTON_SPACE),
            column![
                create_button("CE", Message::CE, EtrStyle),
                create_button("7", Message::Num(Seven), NumStyle),
                create_button("4", Message::Num(Four), NumStyle),
                create_button("1", Message::Num(One), NumStyle),
                create_button(".", Message::Num(Dot), OpsStyle),
            ]
            .spacing(BUTTON_SPACE)
            .align_items(Alignment::Center),
            column![
                create_button("C", Message::C, EtrStyle),
                create_button("8", Message::Num(Eight), NumStyle),
                create_button("5", Message::Num(Five), NumStyle),
                create_button("2", Message::Num(Two), NumStyle),
                create_button("0", Message::Num(Zero), NumStyle),
            ]
            .spacing(BUTTON_SPACE),
            column![
                create_button("DEL", Message::DEL, EtrStyle),
                create_button("9", Message::Num(Nine), NumStyle),
                create_button("6", Message::Num(Six), NumStyle),
                create_button("3", Message::Num(Three), NumStyle),
                create_button("=", Message::Enter, EtrStyle),
            ]
            .spacing(BUTTON_SPACE),
        ]
        .spacing(BUTTON_SPACE)
        .align_items(Alignment::Center);

        let screen_style: fn(&Theme) -> container::Appearance = screen_container_style;
        let screen = container(
            column![text(&self.history).size(22), text(&self.display).size(50)]
                .spacing(5)
                .align_items(Alignment::End),
        )
        .width(279)
        .height(120)
        .center_y()
        .align_x(Horizontal::Right)
        .padding(20)
        .style(screen_style);

        let background: fn(&Theme) -> container::Appearance = calculator_background_style;
        container(column![screen, keyboard].spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(background)
            .into()
    }
}
