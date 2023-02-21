#![windows_subsystem = "windows"]

use iced::window::icon::Icon;
use iced::window::Settings as WindowSettings;
use iced::Sandbox;
use iced::Settings;
use image::ImageFormat;
use mini_calculator::Calculator;

fn main() -> iced::Result {
    let my_icon = include_bytes!("icon.png");

    Calculator::run(Settings {
        window: WindowSettings {
            resizable: false,
            size: (300, 430),
            icon: Some(Icon::from_file_data(my_icon, Some(ImageFormat::Png)).unwrap()),
            ..WindowSettings::default()
        },
        ..Settings::default()
    })
}
