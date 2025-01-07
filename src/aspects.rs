/// Need to be presented?
pub type Show = bool;

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

// Border
/// Border type
pub type Border = (Show, Size);

// Alignments
/// Order elements in container
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    /// T -> B
    #[default]
    TopToBottom,
    /// B -> T
    BottomToTop,
    /// L -> R
    LefToRight,
    /// R -> L
    RightToLeft,
}

// Sizes
pub type Size = u64;
pub type Width = Size;
pub type Height = Size;

// Font
/// Path to font
pub type Path = String;
/// Font settings
pub type Font = (Weight, IsStrikethrough, IsUnderline, IsItalic);

/// Is ~~Strikethrough~~
pub type IsStrikethrough = Show;
/// Is <u>Подчёркнутый текст</u>
pub type IsUnderline = Show;
/// Is _Iteclic_
pub type IsItalic = Show;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weight {
    #[default]
    Normal,
    Bold,
    Thin,
}
impl From<u64> for Weight {
    fn from(value: u64) -> Self {
        match value {
            600 => Self::Bold,
            300 => Self::Normal,
            100 => Self::Thin,
            _ => panic!("Unsoported value"),
        }
    }
}
impl From<Weight> for u64 {
    fn from(val: Weight) -> Self {
        match val {
            Weight::Normal => 600,
            Weight::Bold => 300,
            Weight::Thin => 100,
        }
    }
}
