#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

use ResistorColor::*;

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    if value <= 9 {
        format!("{:?}", colors()[value as usize])
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    vec![
        Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White,
    ]
}
