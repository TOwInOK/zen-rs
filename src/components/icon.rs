use crate::aspects::{BackgroundColor, ForegroundColor, Height, Path, Width};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Svg
pub struct Icon {
    // content
    content: Vec<Path>,
    // Aspects
    // colors
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
    // size
    width: Width,
    height: Height,
}
