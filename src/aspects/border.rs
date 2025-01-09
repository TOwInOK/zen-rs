//! Line/Border aspects types

use super::{Color, Size};

// Border
/// Border type
pub type Border = (Size, Color, Radius);
pub type Radius = Size;
