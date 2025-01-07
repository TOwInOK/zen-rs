/// Need to be presented?
pub type Show = bool;
/// Path to any destination
pub type Path = String;
/// Size
pub type Size = u64;

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

// Link
pub type Link = Option<Path>;

// Sizes
pub type Width = Size;
pub type Height = Size;

// Spaces
/// Space between components
pub type Gap = Size;
/// Space between content and edge
pub type Padding = Size;

// Font
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
