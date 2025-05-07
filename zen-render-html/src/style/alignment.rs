use zen_core::prelude::AlignmentPrimitive;

pub fn alignment(attr: AlignmentPrimitive) -> String {
    format!(
        "align-items: {};",
        match attr {
            AlignmentPrimitive::Left => "start",
            AlignmentPrimitive::Center => "center",
            AlignmentPrimitive::Right => "end",
            AlignmentPrimitive::Stretch => "stretch",
        }
    )
}
