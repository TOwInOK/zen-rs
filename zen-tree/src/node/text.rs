use zen_core::prelude::*;

use crate::size::GetSize;

#[derive(Debug, Default, Clone)]
pub struct Text {
    // Local content
    /// Paths
    pub content: String,
    /// Font family
    pub font_family: String,

    // Aspects
    /// Background color
    pub bg: RGBA,
    /// Foreground color
    pub fg: RGBA,
    /// Font size
    pub size: SizePremitive,
    /// Padding around components
    pub padding: SizePremitive,
}

pub fn text() -> Text {
    Text::default()
}

impl Text {
    pub fn content(
        mut self,
        content: impl ToString,
    ) -> Self {
        self.content = content.to_string();
        self
    }
    /// set font family
    pub fn font_family(
        mut self,
        font_family: impl ToString,
    ) -> Self {
        self.font_family = font_family.to_string();
        self
    }
}

impl Background for Text {
    fn background(
        mut self,
        value: RGBA,
    ) -> Self {
        self.bg = value;
        self
    }
}

impl Foreground for Text {
    /// Set border color
    fn foreground(
        mut self,
        value: RGBA,
    ) -> Self {
        self.fg = value;
        self
    }
}

impl Size for Text {
    /// set font size
    fn width(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.size = value;
        self
    }
    /// set font size
    fn height(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.size = value;
        self
    }
}

impl Padding for Text {
    fn padding(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.padding = value;
        self
    }
}

impl GetSize for Text {
    fn get_size(&self) -> (SizePremitive, SizePremitive) {
        (self.size + self.padding, self.size + self.padding)
    }
}
