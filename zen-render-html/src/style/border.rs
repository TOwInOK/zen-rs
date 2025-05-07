//! https://developer.mozilla.org/en-US/docs/Web/CSS/border

use zen_core::{aspect::RGBA, prelude::BorderParts};

pub fn border(
    color: RGBA,
    radius: BorderParts,
    size: BorderParts,
) -> String {
    format!(
        "border-color: rgba({}, {}, {}, {}); border-radius: {} {} {} {}; border-width: {} {} {} {};",
        color.0,
        color.1,
        color.2,
        color.3,
        radius.0,
        radius.1,
        radius.2,
        radius.3,
        size.0,
        size.1,
        size.2,
        size.3
    )
}
