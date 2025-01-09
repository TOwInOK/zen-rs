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
/// Only for text
///
/// **note** border has self color
pub type ForegroundColor = Color;
/// Only for background
pub type BackgroundColor = Color;

/// Choose svg color
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
