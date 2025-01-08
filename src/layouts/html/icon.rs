use crate::components::icon::Icon;
// TODO: Create type for color for ICON
// - CurrenColor
// - None
// - HEX
pub fn icon_html(component: &Icon) -> String {
    // data
    let fg = component.get_foreground_color().to_string();
    let bg = component.get_background_color().to_string();
    let w = component.get_width();
    let h = component.get_height();
    let (bl, bt, bb, br) = component.get_view_box();

    // content
    let content = component.get_content();

    // content formating
    let first_path_wrap = |path: &str| format!(r#"<path stroke="none" d="{path}" "fill="{bg}" />"#);
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
        xmlns="http://www.w3.org/2000/svg"
        width="{w}"
        height="{h}"
        {vb}
        fill="{fg}"
        {slp}
        {slj}
        {sw}
        {first}
        {paths}
    "#
    )
}
