use tagu::{
    build,
    elem::{DynamicElement, Elem},
};
use zen_tree::node::text::Text;

use crate::style::{
    background::background_color, font_family::font_family, foreground::foreground_color,
    size::font_size, spacing::padding,
};

pub fn text(node: &Text) -> DynamicElement<'_> {
    build::box_elem(
        build::elem("div")
            .with(("style", style(node)))
            .inline()
            .append(build::raw(&node.content)),
    )
}

fn style(node: &Text) -> String {
    let s = [
        font_size(node.size),
        background_color(node.bg),
        foreground_color(node.fg),
        padding(node.padding),
        font_family(&node.font_family),
    ];
    s.join("")
}
