//! This module contains the `icon_html` function for rendering an `Icon` component
//! into an SVG string with specified styles and attributes.
//!
//! The `icon_html` function takes an `Icon` component, extracts its properties
//! (such as foreground and background colors, width, height, viewBox, stroke
//! attributes, and paths), and generates an SVG element with the appropriate
//! attributes and content. The paths are wrapped inside `<path>` tags, and
//! the SVG attributes are applied accordingly, such as stroke and fill styles.
//!
//! # Example
//! ```rust
//! let icon = Icon::new()
//!     .foreground_color((255, 0, 0))
//!     .background_color((0, 0, 0))
//!     .width(24)
//!     .height(24)
//!     .content(vec!["M10 10 H 90 V 90 H 10 Z"]);
//! let svg = icon_html(&icon);
//! assert_eq!(svg, r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 100 100" fill="rgb(0, 0, 0)" stroke="rgb(255, 0, 0)" stroke-linecap="butt" stroke-linejoin="miter" stroke-width="1"><path d="M10 10 H 90 V 90 H 10 Z" fill="rgb(0, 0, 0)" /></svg>"#);
//! ```

use crate::components::{icon::Icon, XMLNS};

/// Renders an `Icon` component into an SVG string with the specified attributes
/// such as foreground and background colors, width, height, viewBox, and paths.
///
/// # Arguments
/// * `component` - A reference to an `Icon` component containing the properties
///   such as content (paths), size, stroke attributes, and colors.
///
/// # Returns
/// A string representing the SVG element that renders the icon with the
/// specified properties and styles.
///
/// # Example
/// ```rust
/// let icon = Icon::new()
///     .foreground_color((255, 0, 0))
///     .background_color((0, 0, 0))
///     .width(24)
///     .height(24)
///     .content(vec!["M10 10 H 90 V 90 H 10 Z"]);
/// let svg = icon_html(&icon);
/// assert_eq!(svg, r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 100 100" fill="rgb(0, 0, 0)" stroke="rgb(255, 0, 0)" stroke-linecap="butt" stroke-linejoin="miter" stroke-width="1"><path d="M10 10 H 90 V 90 H 10 Z" fill="rgb(0, 0, 0)" /></svg>"#);
/// ```
pub fn icon_html(component: &Icon) -> String {
    let xmlns = XMLNS;
    // data
    let fg = component.get_foreground_color().to_string();
    let bg = component.get_background_color().to_string();
    let w = component.get_width();
    let h = component.get_height();
    let (bl, bt, bb, br) = component.get_view_box();

    // content
    let content = component.get_content();

    // content formating
    let first_path_wrap = |path: &str| format!(r#"<path stroke="none" d="{path}" fill="{bg}" />"#);
    let path_wrap = |path: &str| format!(r#"<path d="{path}"/>"#);
    let first = if let Some(path) = content.first() {
        first_path_wrap(path)
    } else {
        "".to_string()
    };
    let content = &content[1..];
    let paths: String = content.iter().map(|x| path_wrap(x)).collect();

    // Specific svg attributes
    let slp = component
        .get_stroke_linecap()
        .map(|x| format!(r#"stroke-linecap="{x}""#))
        .unwrap_or_default();
    let slj = component
        .get_stroke_linejoin()
        .map(|x| format!(r#"stroke-linejoin="{x}""#))
        .unwrap_or_default();
    let sw = component
        .get_stroke_width()
        .map(|x| format!(r#"stroke-width="{x}""#))
        .unwrap_or_default();
    let vb = format!(r#"viewBox="{bl} {bt} {bb} {br}""#);

    // out
    format!(
        r#"
        <svg xmlns="{xmlns}"
        width="{w}"
        height="{h}"
        {vb}
        fill="{bg}"
        stroke="{fg}"
        {slp}
        {slj}
        {sw}>
        {first}
        {paths}</svg>
    "#
    )
}
