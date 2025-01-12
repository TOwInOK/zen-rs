//! Line/Border aspects types

use super::{Color, Size};

// Border
/// Border settings
/// - [BorderPart]
/// - [Color]
/// - [Radius]
pub type BorderStyle = (BorderPart, Color, Radius);
/// Radius (in px)
/// - [Size]
pub type Radius = Size;

/// Part of border
///
/// (Left, Top, Bottom, Right)
pub type BorderPart = (Size, Size, Size, Size);
