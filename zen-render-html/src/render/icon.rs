use tagu::{
    build,
    elem::{DynamicElement, Elem},
};
use zen_tree::node::icon::Icon;

use crate::style::{
    background::background_color, foreground::foreground_color, size::size, spacing::padding,
};

pub fn icon(node: &Icon) -> DynamicElement<'_> {
    build::box_elem(
        build::elem("svg")
            .with(("style", style(node)))
            .inline()
            .append(build::raw(&node.content)),
    )
}

fn style(node: &Icon) -> String {
    let s = [
        size(node),
        background_color(node.bg),
        foreground_color(node.fg),
        padding(node.padding),
    ];
    s.join("")
}
