//! Text component
//!
//! Provide
//! - Text
//! - fore/back-ground colors
//! - font style
//! - link (only html/leptos)

use crate::aspects::{
    BackgroundColor, DefaultFontFamily, FontStyle, ForegroundColor, Link, Size, Weight,
};

/// Text representation
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Text {
    // Content
    content: String,
    // Aspects
    // colors
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
    // font settings
    font_style: FontStyle,
    // link
    /// if is Some, this is link
    link: Link,
}

/// Return default text object
pub fn text() -> Text {
    Text::default()
}

impl Text {
    /// Set link
    pub fn link(mut self, link: impl ToString) -> Self {
        self.link = Some(link.to_string());
        self
    }

    /// Set text
    pub fn content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();
        self
    }

    /// Set foreground color
    pub fn foreground_color(mut self, foreground_color: ForegroundColor) -> Self {
        self.foreground_color = foreground_color;
        self
    }

    /// Set background color
    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = background_color;
        self
    }

    /// Set font size
    pub fn size(mut self, size: Size) -> Self {
        self.font_style.0 = size;
        self
    }

    /// just revers bool
    pub fn is_strikeout(mut self) -> Self {
        self.font_style.2 = !self.font_style.2;
        self
    }
    /// just revers bool
    pub fn is_underline(mut self) -> Self {
        self.font_style.3 = !self.font_style.3;
        self
    }
    /// just revers bool
    pub fn is_italic(mut self) -> Self {
        self.font_style.4 = !self.font_style.4;
        self
    }

    /// Set weight of font
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

    /// Set a name font
    pub fn font_custom(mut self, name: impl ToString) -> Self {
        self.font_style.5 .0 = name.to_string();
        self
    }

    /// Set default font-family default type
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

    /// Set all [FontStyle] directly
    pub fn font_style(mut self, font: FontStyle) -> Self {
        self.font_style = font;
        self
    }

    /// Get text
    pub fn get_content(&self) -> &str {
        self.content.as_str()
    }

    /// Get foreground color
    pub fn get_foreground_color(&self) -> ForegroundColor {
        self.foreground_color
    }

    /// Get background color
    pub fn get_background_color(&self) -> BackgroundColor {
        self.background_color
    }

    /// Get font size
    pub fn get_size(&self) -> Size {
        self.font_style.0
    }

    /// Get all font style
    pub fn get_font(&self) -> &FontStyle {
        &self.font_style
    }

    /// Get link
    pub fn get_link(&self) -> &Link {
        &self.link
    }
}
