use zen_core::aspect::RGBA;

pub fn foreground_color(color: RGBA) -> String {
    format!(
        "foreground-color: rgba({}, {}, {}, {});",
        color.0, color.1, color.2, color.3
    )
}
