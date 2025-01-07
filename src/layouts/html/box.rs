use crate::components::r#box::Box;

use super::HtmlBuilder;

pub fn box_html(component: &Box) -> String {
    // data
    let (f_red, f_green, f_blue, f_alpha) = component.get_foreground_color();
    let (b_red, b_green, b_blue, b_alpha) = component.get_background_color();
    let w = component.get_width();
    let h = component.get_height();
    let (is_border, b_size) = component.get_border();
    let gap = component.get_gap();
    let padding = component.get_padding();

    // content
    let content = {
        let mut out = String::new();
        let components = component.get_components();
        for component in components {
            let component = HtmlBuilder::render(component);
            out.push_str(&component);
        }
        out
    };

    // css
    let padding = format!("padding: {padding}px");
    let gap = format!("gap: {gap}px");
    let direction = match component.get_direction() {
        crate::aspects::Order::TopToBottom => "flex; flex-direction: column",
        crate::aspects::Order::BottomToTop => "flex; flex-direction: column-reverse",
        crate::aspects::Order::LefToRight => "flex; flex-direction: row-reverse",
        crate::aspects::Order::RightToLeft => "flex; flex-direction: row",
    };
    let size = {
        let mut out = String::new();
        if *w != 0 {
            out.push_str(&format!("width: {w};"));
        }
        if *h != 0 {
            out.push_str(&format!("height: {h};"));
        }
        out
    };
    let bg_color = format!("background-color:: rgba({b_red}, {b_green}, {b_blue}, {b_alpha});");
    let border = if *is_border {
        format!("border: {b_size}px, rgba({f_red}, {f_green}, {f_blue}, {f_alpha})")
    } else {
        "".to_string()
    };
    // css build
    let style = format!(r#"style="{size} {gap} {direction} {padding} {bg_color} {border}""#);

    // out data
    let out = format!("<div {style}>{content}</div>");
    out
}
