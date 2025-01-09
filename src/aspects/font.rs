//! font aspects types

use std::fmt::Display;

use super::Show;

// Font
/// Font settings
pub type FontStyle = (Weight, IsStrikethrough, IsUnderline, IsItalic, FontFamily);

pub type ApplyableFont = String;

pub type FontFamily = (ApplyableFont, DefaultFontFamily);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DefaultFontFamily {
    // mb this need's to be default, idk
    Serif,
    // mb this true path, idk
    #[default]
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    SystemUi,
}

impl Display for DefaultFontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let font_name = match self {
            DefaultFontFamily::Serif => "serif",
            DefaultFontFamily::SansSerif => "sans-serif",
            DefaultFontFamily::Monospace => "monospace",
            DefaultFontFamily::Cursive => "cursive",
            DefaultFontFamily::Fantasy => "fantasy",
            DefaultFontFamily::SystemUi => "system-ui",
        };
        write!(f, "{}", font_name)
    }
}

/// Is ~~Strikethrough~~
pub type IsStrikethrough = Show;
/// Is <u>Подчёркнутый текст</u>
pub type IsUnderline = Show;
/// Is _Iteclic_
pub type IsItalic = Show;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weight {
    Heavy,
    ExtraBold,
    Bold,
    SemiBold,
    Medium,
    #[default]
    Normal,
    Light,
    ExtraLight,
    Thin,
}
impl From<u64> for Weight {
    fn from(value: u64) -> Self {
        match value {
            900 => Self::Heavy,
            800 => Self::ExtraBold,
            700 => Self::Bold,
            600 => Self::SemiBold,
            500 => Self::Medium,
            400 => Self::Normal,
            300 => Self::Light,
            200 => Self::ExtraLight,
            100 => Self::Thin,
            _ => panic!("Unsoported value"),
        }
    }
}
impl From<Weight> for u64 {
    fn from(val: Weight) -> Self {
        match val {
            Weight::Heavy => 900,
            Weight::ExtraBold => 800,
            Weight::Bold => 700,
            Weight::SemiBold => 600,
            Weight::Medium => 500,
            Weight::Normal => 400,
            Weight::Light => 300,
            Weight::ExtraLight => 200,
            Weight::Thin => 100,
        }
    }
}
