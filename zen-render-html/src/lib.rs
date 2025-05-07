mod render;
mod style;

use core::fmt;

use render::node_render;
use tagu::{build, elem::Elem};
use zen_tree::node::Node;

#[derive(Debug)]
pub struct Html {
    node: Node,
}

impl Html {
    pub fn new(node: Node) -> Self {
        Self { node }
    }
}

impl Html {
    pub fn render(&self) -> Result<String, fmt::Error> {
        let html = build::single("DOCTYPE html")
            .with_start("!")
            .with_ending("")
            .chain(build::elem("html"));
        let out = html.append(node_render(&self.node));
        let mut buff = String::new();
        tagu::render(out.with_tab(" "), &mut buff)?;
        Ok(buff)
    }
}
