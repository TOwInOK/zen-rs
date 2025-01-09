//! Line/Border aspects types

use super::{Color, Size};

// Border
/// Border settings
/// - [Size]
/// - [Color]
/// - [Radius]
pub type Border = (Size, Color, Radius);
/// Radius (in px)
/// - [Size]
pub type Radius = Size;
