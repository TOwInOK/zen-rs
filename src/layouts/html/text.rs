use crate::components::text::Text;

pub fn text_html(component: &Text) -> String {
    // data
    let (f_red, f_green, f_blue, f_alpha) = component.get_foreground_color();
    let (b_red, b_green, b_blue, b_alpha) = component.get_background_color();
    let size = component.get_size();
    let (weight, s, u, i) = component.get_font();

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

    //css
    let font_style = {
        let u64_weight: u64 = (*weight).into();
        match weight {
            crate::aspects::Weight::Heavy => format!("font-weight: {u64_weight};"),
            crate::aspects::Weight::ExtraBold => format!("font-{u64_weight}: {u64_weight};"),
            crate::aspects::Weight::Bold => format!("font-weight: {u64_weight};"),
            crate::aspects::Weight::SemiBold => format!("font-weight: {u64_weight};"),
            crate::aspects::Weight::Medium => format!("font-weight: {u64_weight};"),
            crate::aspects::Weight::Normal => {
                format!("font-weight: {u64_weight};")
            }
            crate::aspects::Weight::Light => format!("font-weight: {u64_weight};"),
            crate::aspects::Weight::ExtraLight => format!("font-weight: {u64_weight};"),
            crate::aspects::Weight::Thin => format!("font-weight: {u64_weight};"),
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

    let css = format!(r#"style="{bg_color} {fg_color} {font_size} {font_sui} {font_style}" "#);
    // out
    format!("<{tag} {href} {css}>{content}</{tag}>")
}
