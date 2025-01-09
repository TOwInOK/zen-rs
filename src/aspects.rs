//! Base types of aspects
//!
//! There are places base aspects
pub mod border;
pub mod color;
pub mod font;
pub mod order;
pub mod spaceing;
pub mod svg;

pub use border::*;
pub use color::*;
pub use font::*;
pub use order::*;
pub use spaceing::*;
pub use svg::*;

/// Need to be presented?
pub type Show = bool;
/// Path to any destination
pub type Path = String;
/// Size
pub type Size = u64;

// Link
pub type Link = Option<Path>;

// Sizes
pub type Width = Size;
pub type Height = Size;
