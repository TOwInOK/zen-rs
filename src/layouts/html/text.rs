//! This module contains the `text_html` function for rendering text components
//! into HTML with custom styles, as well as a test module for validating the
//! function's behavior.
//!
//! The `text_html` function takes a `Text` component and generates a complete
//! HTML tag (either a `<div>` or `<a>`) with applied CSS styles for foreground
//! and background colors, font size, weight, style, and family. It also supports
//! optional linking by generating a valid `href` attribute when a link is provided.
//!
//! The test module ensures that the `text_html` function works as expected by
//! rendering a few `Text` components and printing the resulting HTML.

use crate::components::text::Text;

/// Renders a `Text` component into an HTML string with applied styles.
///
/// This function takes a `Text` component, extracts its properties (such as
/// foreground color, background color, font size, weight, and optional link),
/// and generates an HTML tag with the appropriate CSS styles.
///
/// # Arguments
/// * `component` - A reference to a `Text` component that contains the content,
///   colors, font properties, and optionally a link.
///
/// # Returns
/// A string representing the HTML tag for the component with the correct styles.
///
/// # Example
/// ```rust
/// let text_component = Text::new()
///     .content("Hello, World!")
///     .foreground_color((255, 0, 0, 1))
///     .background_color((0, 0, 0, 0))
///     .size(16);
/// let html = text_html(&text_component);
/// assert_eq!(html, r#"<div style="color: rgba(255, 0, 0, 1); background-color: rgba(0, 0, 0, 0); font-size: 16px; font-family: 'CustomFont', Arial;">Hello, World!</div>"#);
/// ```
pub fn text_html(component: &Text) -> String {
    // data
    let (f_red, f_green, f_blue, f_alpha) = component.get_foreground_color();
    let (b_red, b_green, b_blue, b_alpha) = component.get_background_color();
    let (size, weight, s, u, i, (custom_font, default_font)) = component.get_font();

    // tag
    let tag = if component.get_link().is_some() {
        "a"
    } else {
        "div"
    };

    // link (href)
    let href = if let Some(href) = component.get_link() {
        format!(r#"href="{href}""#)
    } else {
        "".to_string()
    };

    // content
    let content = component.get_content();

    // css
    let font_style = {
        let u64_weight: u64 = (*weight).into();
        let fw = |s: u64| format!("font-weight: {s};");
        match weight {
            crate::aspects::Weight::Heavy => fw(u64_weight),
            crate::aspects::Weight::ExtraBold => fw(u64_weight),
            crate::aspects::Weight::Bold => fw(u64_weight),
            crate::aspects::Weight::SemiBold => fw(u64_weight),
            crate::aspects::Weight::Medium => fw(u64_weight),
            crate::aspects::Weight::Normal => fw(u64_weight),
            crate::aspects::Weight::Light => fw(u64_weight),
            crate::aspects::Weight::ExtraLight => fw(u64_weight),
            crate::aspects::Weight::Thin => fw(u64_weight),
        }
    };
    let font_size = format!("font-size: {size}px;");
    let font_sui = {
        let mut font_sui = String::new();
        if *i {
            font_sui.push_str("font-style: italic;")
        }
        if *u {
            font_sui.push_str("text-decoration-line: underline;")
        }
        if *s {
            font_sui.push_str("text-decoration-line: line-through;")
        }
        font_sui
    };
    let bg_color = format!("background-color: rgba({b_red}, {b_green}, {b_blue}, {b_alpha});");
    let fg_color = format!("color: rgba({f_red}, {f_green}, {f_blue}, {f_alpha});");
    let font_family = format!(r#"font-family: '{custom_font}', {default_font};"#);

    let css = format!(
        r#"style="{bg_color} {fg_color} {font_size} {font_sui} {font_style} {font_family}""#
    );
    // out
    format!("<{tag} {css} {href}>{content}</{tag}>")
}

#[cfg(test)]
mod test_text {
    use crate::components::text::text;

    use super::text_html;

    #[test]
    fn def() {
        let text1 = text()
            .content("Some Text")
            .background_color((255, 255, 255, 100))
            .foreground_color((0, 0, 0, 100))
            .size(18);
        let html = text_html(&text1);
        println!("OUT: {}", html);
        let text2 = text()
            .content("Some Text")
            .background_color((255, 255, 255, 100))
            .foreground_color((0, 0, 0, 100))
            .size(18)
            .link("#");
        let html = text_html(&text2);
        println!("OUT: {}", html);
    }
}
