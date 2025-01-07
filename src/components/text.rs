use crate::aspects::{BackgroundColor, Font, ForegroundColor, Size};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text {
    // Content
    content: String,
    // Aspects
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
    size: Size,
    font: Font,
}

pub fn text() -> Text {
    Text::default()
}

impl Text {
    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn foreground_color(mut self, foreground_color: ForegroundColor) -> Self {
        self.foreground_color = foreground_color;
        self
    }

    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn get_content(&self) -> &str {
        self.content.as_str()
    }

    pub fn get_foreground_color(&self) -> ForegroundColor {
        self.foreground_color
    }

    pub fn get_background_color(&self) -> BackgroundColor {
        self.background_color
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_font(&self) -> &Font {
        &self.font
    }
}
