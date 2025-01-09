//! # Library of Components for Generating Non-Interactive Pages/Cards/Files/Content
//!
//! ## Components
//! * [x] Container
//! * [x] Text
//!   - [x] As text (HTML `<div>`)
//!   - [x] As link (HTML , `<a>`)
//! * [x] Icon
//!
//! ## Available Renders
//! * [x] HTML (custom implementation)
//! * [ ] PNG (conversion from SVG)
//! * [ ] PDF (Skia)
//! * [ ] SVG (Skia)
//! * [ ] Leptos
//!
//! ## Goals
//! - Provide the minimal required support for renderer-specific features
//! - Ensure maximum compatibility of element properties across all renderers
//! - Offer a minimally comfortable set of components
//!
//! ## Non-Goals
//! - Full support for all available properties in all renderers
//! - Interactivity (e.g., CSS animations)
//! - Script support for HTML

pub mod aspects;
pub mod components;
pub mod layouts;
