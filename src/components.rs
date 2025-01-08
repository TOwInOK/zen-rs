use container::Container;
use icon::Icon;
use text::Text;

pub mod container;
pub mod icon;
pub mod text;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Components {
    Container(Container),
    Text(Text),
    Icon(Icon),
}

impl From<Container> for Components {
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}

impl From<Text> for Components {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Icon> for Components {
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}

impl Default for Components {
    fn default() -> Self {
        Self::Container(Container::default())
    }
}
