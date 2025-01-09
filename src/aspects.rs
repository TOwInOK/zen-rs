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
///
/// helpfull arugments in place of Option type.
///
/// **note**: mb will be deleted in future
pub type Show = bool;

/// Path to any destination
pub type Path = String;

/// Size
///
/// By default means only px, but it can be different for some specific layout
pub type Size = u64;

/// Link
///
/// *note*: Only HTML/Leptos Attribute and it will be ignored for another render type
///
/// HTML Href
pub type Link = Option<Path>;

// Sizes
/// Width
/// presumably in pixels
pub type Width = Size;

/// Height
/// presumably in pixels
pub type Height = Size;
