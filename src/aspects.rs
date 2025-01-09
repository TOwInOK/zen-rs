//! Base types of aspects
//!
//! This module defines base aspects and their associated types.

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

/// Indicates whether an element should be displayed.
///
/// This is a helper type that serves as a replacement for `Option` in certain cases.
///
/// **Note**: This type may be removed in future versions.
pub type Show = bool;

/// Represents a path to a destination.
pub type Path = String;

/// Represents a size value.
///
/// By default, size is assumed to be in pixels, but specific layouts may interpret it differently.
pub type Size = u64;

/// Represents a hyperlink.
///
/// **Note**: This is applicable only for HTML/Leptos render attributes and will be ignored for other render types.
///
/// For example, this corresponds to an HTML `href` attribute.
pub type Link = Option<Path>;

// Size-related types
/// Represents the width of an element.
/// Presumably measured in pixels.
pub type Width = Size;

/// Represents the height of an element.
/// Presumably measured in pixels.
pub type Height = Size;
