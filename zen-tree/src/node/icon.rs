use zen_core::prelude::*;

use crate::size::GetSize;

#[derive(Debug, Default, Clone)]
pub struct Icon {
    // Local content
    /// Paths
    pub content: String,

    // Aspects
    /// Background color
    pub bg: RGBA,
    /// Foreground color
    pub fg: RGBA,
    /// Width & Height
    pub size: (SizePremitive, SizePremitive),
    /// Padding around components
    pub padding: SizePremitive,
}

pub fn icon() -> Icon {
    Icon::default()
}

impl Icon {
    pub fn content(
        mut self,
        content: impl ToString,
    ) -> Self {
        self.content = content.to_string();
        self
    }
}

impl Background for Icon {
    fn background(
        mut self,
        value: RGBA,
    ) -> Self {
        self.bg = value;
        self
    }
}

impl Foreground for Icon {
    /// Set border color
    fn foreground(
        mut self,
        value: RGBA,
    ) -> Self {
        self.fg = value;
        self
    }
}

impl Size for Icon {
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

impl Padding for Icon {
    fn padding(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.padding = value;
        self
    }
}

impl GetSize for Icon {
    fn get_size(&self) -> (SizePremitive, SizePremitive) {
        (self.size.0 + self.padding, self.size.1 + self.padding)
    }
}
