//! This module contains the `container_html` function for rendering a `Container`
//! component into an HTML string with specified styles and content.
//!
//! The `container_html` function takes a `Container` component, extracts its properties
//! (such as background color, size, border, gap, padding, alignment, and direction),
//! and generates an HTML `<div>` element. The function applies appropriate CSS styles
//! for the container layout (including flexbox styles) and recursively renders
//! its child components into the container.

use super::HtmlBuilder;
use crate::components::container::Container;

/// Renders a `Container` component into an HTML string with the specified styles
/// such as background color, size, border, padding, alignment, and flexbox layout.
///
/// # Arguments
/// * `component` - A reference to a `Container` component containing the properties
///   such as background color, dimensions, border, padding, and alignment.
///   It may also include nested components that are rendered inside the container.
///
/// # Returns
/// A string representing the HTML `<div>` element with the specified properties,
/// styles, and content (child components).
pub fn container_html(component: &Container) -> String {
    // data
    let (bg_red, bg_green, bg_blue, bg_alpha) = component.get_background_color();
    let w = component.get_width();
    let h = component.get_height();
    let ((b_size_l, b_size_t, b_size_b, b_size_r), (b_red, b_green, b_blue, b_alpha), b_radius) = component.get_border();
    let gap = component.get_gap();
    let padding = component.get_padding();
    let align = component.get_align_content();

    // local state
    let mut is_col = false;

    // content
    let content = {
        let mut out = String::new();
        let components = component.get_components();
        for component in components {
            let component = HtmlBuilder::render_component(component);
            out.push_str(&component);
        }
        out
    };

    // css

    let padding = format!("padding: {padding}px;");
    let gap = format!("gap: {gap}px;");
    let flex = if *component.get_flex() {
        "display: flex;"
    } else {
        "display: block;"
    };
    let direction = {
        let dn = match component.get_direction() {
            crate::aspects::Order::TopToBottom => {
                is_col = !is_col;
                "column"
            }
            crate::aspects::Order::BottomToTop => "column-reverse",
            crate::aspects::Order::LefToRight => "row",
            crate::aspects::Order::RightToLeft => "row-reverse",
        };
        format!("flex-direction: {dn};")
    };
    let align_content = {
        let a = match align {
            crate::aspects::Align::Left => "flex-left",
            crate::aspects::Align::Center => "center",
            crate::aspects::Align::Right => "flex-end",
            crate::aspects::Align::SpaceBetween => "space-between",
        };
        if is_col {
            format!("align-items: {a};")
        } else {
            format!("justify-content: {a};")
        }
    };
    let align_items = {
        let a = match align {
            crate::aspects::Align::Left => "flex-left",
            crate::aspects::Align::Center => "center",
            crate::aspects::Align::Right => "flex-end",
            crate::aspects::Align::SpaceBetween => "space-between",
        };
        if is_col {
            format!("justify-content: {a};")
        } else {
            format!("align-items: {a};")
        }
    };
    let size = {
        let mut out = String::new();
        if *component.get_width_full() {
            out.push_str("width: 100%;");
        } else if *w != 0 {
            out.push_str(&format!("width: {w};"));
        }
        if *component.get_height_full() {
            out.push_str("height: 100%;");
        } else if *h != 0 {
            out.push_str(&format!("height: {h};"));
        }

        out
    };
    let bg_color = format!("background-color: rgba({bg_red}, {bg_green}, {bg_blue}, {bg_alpha});");
    let border = format!(
        "border-left: {b_size_l}px solid rgba({b_red}, {b_green}, {b_blue}, {b_alpha});
        border-top: {b_size_t}px solid rgba({b_red}, {b_green}, {b_blue}, {b_alpha});
        border-bottom: {b_size_b}px solid rgba({b_red}, {b_green}, {b_blue}, {b_alpha});
        border-right: {b_size_r}px solid rgba({b_red}, {b_green}, {b_blue}, {b_alpha});
        border-radius: {b_radius}px;");
    // css build
    let style = format!(
        r#"style="{flex} {align_content} {align_items} {size} {gap} {direction} {padding} {bg_color} {border}""#
    );

    // out data
    let out = format!("<div {style}>{content}</div>");
    out
}
