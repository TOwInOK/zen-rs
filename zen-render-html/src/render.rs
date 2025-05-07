use container::container;
use icon::icon;
use link::link;
use tagu::elem::DynamicElement;
use text::text;
use zen_tree::node::Node;

pub mod container;
pub mod icon;
pub mod link;
pub mod text;

pub fn node_render(node: &Node) -> DynamicElement<'_> {
    match node {
        Node::Container(node) => container(node),
        Node::Icon(node) => icon(node),
        Node::Link(node) => link(node),
        Node::Text(node) => text(node),
    }
}
