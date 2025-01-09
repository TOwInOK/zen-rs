//! color aspects types

use std::fmt::Display;

// Colors
pub type Red = u8;
pub type Green = u8;
pub type Blue = u8;
pub type Alpha = u8;
/// - Red
/// - Green
/// - Blue
/// - Aplha
pub type Color = (Red, Green, Blue, Alpha);
/// For border and text
pub type ForegroundColor = Color;
/// only For Background
pub type BackgroundColor = Color;
/// Hex
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SvgColor {
    #[default]
    None,
    CurrentColor,
    Color(String),
}
impl Display for SvgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SvgColor::None => write!(f, "None"),
            SvgColor::CurrentColor => write!(f, "currentColor"),
            SvgColor::Color(color) => write!(f, "{}", color),
        }
    }
}
