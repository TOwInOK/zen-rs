use r#box::Box;
use text::Text;

pub mod r#box;
pub mod icon;
pub mod text;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Components {
    Box(Box),
    Text(Text),
    Icon,
}

impl Default for Components {
    fn default() -> Self {
        Self::Box(Box::default())
    }
}
