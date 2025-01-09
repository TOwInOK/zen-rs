//! Text component
//!
//! Provide
//! - Text
//! - fore/back-ground colors
//! - font style
//! - link (only html/leptos)

pub mod h;

use crate::aspects::{
    BackgroundColor, DefaultFontFamily, FontStyle, ForegroundColor, Link, Size, Weight,
};

/// Return default [Text] instance
pub fn text() -> Text {
    Text::default()
}

/// Text representation
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Text {
    /// Content of the text component
    content: String,
    /// Foreground color of the text
    foreground_color: ForegroundColor,
    /// Background color of the text
    background_color: BackgroundColor,
    /// Font style of the text (size, weight, etc.)
    font_style: FontStyle,
    /// Optional link associated with the text
    link: Link,
}

impl Text {
    /// Set a link for the text component
    pub fn link(mut self, link: impl ToString) -> Self {
        self.link = Some(link.to_string());
        self
    }

    /// Set the content of the text component
    pub fn content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();
        self
    }

    /// Set the foreground color of the text
    pub fn foreground_color(mut self, foreground_color: ForegroundColor) -> Self {
        self.foreground_color = foreground_color;
        self
    }

    /// Set the background color of the text
    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = background_color;
        self
    }

    /// Set the font size of the text
    pub fn size(mut self, size: Size) -> Self {
        self.font_style.0 = size;
        self
    }

    /// Toggle the strikeout style for the text
    pub fn is_strikeout(mut self) -> Self {
        self.font_style.2 = !self.font_style.2;
        self
    }

    /// Toggle the underline style for the text
    pub fn is_underline(mut self) -> Self {
        self.font_style.3 = !self.font_style.3;
        self
    }

    /// Toggle the italic style for the text
    pub fn is_italic(mut self) -> Self {
        self.font_style.4 = !self.font_style.4;
        self
    }

    /// Set the weight of the font
    ///  - [Weight::Heavy] => 900
    ///  - [Weight::ExtraBold] => 800
    ///  - [Weight::Bold] => 700
    ///  - [Weight::SemiBold] => 600
    ///  - [Weight::Medium] => 500 | Default
    ///  - [Weight::Normal] => 400
    ///  - [Weight::Light] => 300
    ///  - [Weight::ExtraLight] => 200
    ///  - [Weight::Thin] => 100
    pub fn font_weight(mut self, weight: Weight) -> Self {
        self.font_style.1 = weight;
        self
    }

    /// Set a custom font name
    pub fn font_custom(mut self, name: impl ToString) -> Self {
        self.font_style.5 .0 = name.to_string();
        self
    }

    /// Set the default font family type
    /// - Serif
    /// - SansSerif | Default
    /// - Monospace
    /// - Cursive
    /// - Fantasy
    /// - SystemUi
    pub fn font_default(mut self, def: DefaultFontFamily) -> Self {
        self.font_style.5 .1 = def;
        self
    }

    /// Set the complete [FontStyle] directly
    pub fn font_style(mut self, font: FontStyle) -> Self {
        self.font_style = font;
        self
    }

    /// Get the text content
    pub fn get_content(&self) -> &str {
        self.content.as_str()
    }

    /// Get the foreground color
    pub fn get_foreground_color(&self) -> ForegroundColor {
        self.foreground_color
    }

    /// Get the background color
    pub fn get_background_color(&self) -> BackgroundColor {
        self.background_color
    }

    /// Get the font size
    pub fn get_size(&self) -> Size {
        self.font_style.0
    }

    /// Get the complete font style
    pub fn get_font(&self) -> &FontStyle {
        &self.font_style
    }

    /// Get the link (if any) associated with the text
    pub fn get_link(&self) -> &Link {
        &self.link
    }
}
