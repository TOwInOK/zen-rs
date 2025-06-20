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

    fn size(
        self,
        value: SizePremitive,
    ) -> Self
    where
        Self: Sized,
    {
        self.width(value)
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
        // Calculate width based on text content length and font size
        // A rough estimation: each character is approximately 0.6 times the font size in width
        let text_width = if self.content.is_empty() {
            0.0
        } else {
            // Estimate based on character count and font size
            let char_count = self.content.chars().count() as f32;
            char_count * self.size * 0.6
        };

        // Add padding to both dimensions
        let width = text_width + (self.padding * 2.0);
        let height = self.size + (self.padding * 2.0);

        (width, height)
    }
}
