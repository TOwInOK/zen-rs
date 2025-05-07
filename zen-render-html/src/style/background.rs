use zen_core::aspect::RGBA;

pub fn background_color(color: RGBA) -> String {
    format!(
        "background-color: rgba({}, {}, {}, {});",
        color.0, color.1, color.2, color.3
    )
}
