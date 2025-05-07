use container::Container;
use icon::Icon;
use text::Text;

pub mod container;
pub mod icon;
pub mod text;

#[derive(Debug, Clone)]
pub enum Node {
    Container(container::Container),
    Icon(icon::Icon),
    Text(text::Text),
}

impl From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}

impl From<Container> for Node {
    /// Converts a `Container` into a `Node` variant.
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}

impl From<Text> for Node {
    /// Converts a `Text` into a `Node` variant.
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Icon> for Node {
    /// Converts an `Icon` into a `Node` variant.
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}
