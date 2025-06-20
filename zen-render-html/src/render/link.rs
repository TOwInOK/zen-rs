use tagu::{
    build,
    elem::{DynamicElement, Elem},
};
use zen_tree::node::link::Link;

use crate::style::{
    background::background_color, foreground::foreground_color, size::size, spacing::padding,
};

use super::node_render;

pub fn link(node: &Link) -> DynamicElement<'_> {
    build::box_elem(
        build::elem("a")
            .with(("style", style(node)))
            .with(("href", &node.destination))
            .inline()
            .chain(node_render(&node.content)),
    )
}

fn style(node: &Link) -> String {
    let s = [
        size(node),
        background_color(node.bg),
        foreground_color(node.fg),
        padding(node.padding),
    ];
    s.join("")
}
