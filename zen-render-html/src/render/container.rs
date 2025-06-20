use tagu::{
    build,
    elem::{DynamicElement, Elem},
};
use zen_tree::node::container::Container;

use crate::style::{
    alignment::alignment,
    background::background_color,
    border::border,
    direction::direction,
    size::size,
    spacing::{gap, padding},
};

use super::node_render;

pub fn container(node: &Container) -> DynamicElement<'_> {
    build::box_elem(
        build::elem("div")
            .with(("style", style(node)))
            .inline()
            .append(build::from_iter(
                node.content.iter().map(|x| node_render(x)),
            )),
    )
}

fn style(node: &Container) -> String {
    let s = [
        size(node),
        background_color(node.bg),
        border(node.border_color, node.border_radius, node.border_size),
        padding(node.padding),
        gap(node.gap),
        direction(node.direction),
        alignment(node.alignment),
    ];
    s.join("")
}
