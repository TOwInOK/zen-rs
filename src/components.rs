//! Module providing the `Components` enum and its implementations.
//!
//! The `Components` enum acts as a unified abstraction for different UI elements such as containers, text, and icons.
//! This module also includes conversions and a default implementation for `Components`.

pub mod container;
pub mod icon;
pub mod text;

pub use container::*;
pub use icon::*;
pub use text::*;

/// Represents different types of UI components.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Components {
    /// Container component.
    Container(Container),
    /// Text component.
    Text(Text),
    /// Icon component.
    Icon(Icon),
}

impl From<&Components> for Components {
    fn from(value: &Components) -> Self {
        value.clone()
    }
}

impl From<Container> for Components {
    /// Converts a `Container` into a `Components` variant.
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}

impl From<Text> for Components {
    /// Converts a `Text` into a `Components` variant.
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Icon> for Components {
    /// Converts an `Icon` into a `Components` variant.
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}

impl Default for Components {
    /// Returns a default `Components` variant, which is a `Container`.
    fn default() -> Self {
        Self::Container(Container::default())
    }
}
