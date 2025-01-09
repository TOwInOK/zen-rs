//! Svg aspects types

use std::fmt::Display;

/// The stroke-linejoin attribute is a presentation attribute defining the shape to be used at the corners of paths when they are stroked.
///
/// [MDM](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linejoin)
///
/// arcs | bevel |miter | miter-clip | round
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrokeLinejoin {
    Arcs,
    Bevel,
    #[default]
    Miter,
    MiterClip,
    Round,
}
impl Display for StrokeLinejoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrokeLinejoin::Arcs => write!(f, "arcs"),
            StrokeLinejoin::Bevel => write!(f, "bevel"),
            StrokeLinejoin::Miter => write!(f, "miter"),
            StrokeLinejoin::MiterClip => write!(f, "miterClip"),
            StrokeLinejoin::Round => write!(f, "round"),
        }
    }
}

/// The stroke-linecap attribute is a presentation attribute defining the shape to be used at the end of open subpaths when they are stroked.
///
/// [MDM](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linecap)
///
/// butt | round | square
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrokeLinecap {
    #[default]
    Butt,
    Round,
    Square,
}

impl Display for StrokeLinecap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrokeLinecap::Butt => write!(f, "butt"),
            StrokeLinecap::Round => write!(f, "round"),
            StrokeLinecap::Square => write!(f, "square"),
        }
    }
}
