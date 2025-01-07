use r#box::Box;

pub mod r#box;
pub mod icon;
pub mod text;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Components {
    Box(Box),
    Text,
    Icon,
}

impl Default for Components {
    fn default() -> Self {
        Self::Box(Box::default())
    }
}
