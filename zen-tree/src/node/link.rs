use zen_core::prelude::*;

use crate::size::GetSize;

use super::Node;

#[derive(Debug, Default, Clone)]
pub struct Link {
    // Local content
    /// inner node
    content: Box<Node>,
    /// Link destination
    destination: String,

    // Aspects
    /// Background color
    bg: RGBA,
    /// Foreground color
    fg: RGBA,
    /// Width & Height
    size: (SizePremitive, SizePremitive),
    /// Padding around components
    padding: SizePremitive,
}

pub fn link() -> Link {
    Link::default()
}

impl Link {
    pub fn content(
        mut self,
        content: impl Into<Node>,
    ) -> Self {
        self.content = Box::new(content.into());
        self
    }

    pub fn destination(
        mut self,
        destination: impl ToString,
    ) -> Self {
        self.destination = destination.to_string();
        self
    }
}

impl Background for Link {
    fn background(
        mut self,
        value: RGBA,
    ) -> Self {
        self.bg = value;
        self
    }
}

impl Foreground for Link {
    /// Set border color
    fn foreground(
        mut self,
        value: RGBA,
    ) -> Self {
        self.fg = value;
        self
    }
}

impl Size for Link {
    fn width(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.size.0 = value;
        self
    }

    fn height(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.size.1 = value;
        self
    }
}

impl Padding for Link {
    fn padding(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.padding = value;
        self
    }
}

impl GetSize for Link {
    fn get_size(&self) -> (SizePremitive, SizePremitive) {
        let content_size = self.content.get_size();
        (
            self.size.0 + self.padding + content_size.0,
            self.size.1 + self.padding + content_size.1,
        )
    }
}
