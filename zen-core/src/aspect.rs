mod alignment;
mod background;
mod border;
mod direction;
mod foreground;
mod size;
mod spacing;

/// Prelude module for importing all aspects.
pub mod prelude {
    // Premitives
    pub use super::RGBA;
    pub use super::SizePremitive;
    pub use super::alignment::AlignmentPrimitive;
    pub use super::border::BorderParts;
    pub use super::direction::DirectionPremitive;
    // Aspects
    pub use super::alignment::Alignment;
    pub use super::background::Background;
    pub use super::border::Border;
    pub use super::direction::Direction;
    pub use super::foreground::Foreground;
    pub use super::size::Size;
    pub use super::spacing::Gap;
    pub use super::spacing::Padding;
}

pub type SizePremitive = f32;
pub type RGBA = (f32, f32, f32, f32);

// TODO: Add ToString to RGBA
