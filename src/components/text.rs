use crate::aspects::{
    BackgroundColor, DefaultFontFamily, FontStyle, ForegroundColor, Link, Size, Weight,
};

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Text {
    // Content
    content: String,
    // Aspects
    // colors
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
    // size
    size: Size,
    // font
    font_style: FontStyle,
    // link
    /// if is Some, this is link
    link: Link,
}

pub fn text() -> Text {
    Text::default()
}

impl Text {
    pub fn link(mut self, link: impl ToString) -> Self {
        self.link = Some(link.to_string());
        self
    }

    pub fn content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();
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

    /// just revers bool
    pub fn is_strikeout(mut self) -> Self {
        self.font_style.1 = !self.font_style.1;
        self
    }
    /// just revers bool
    pub fn is_underline(mut self) -> Self {
        self.font_style.2 = !self.font_style.2;
        self
    }
    /// just revers bool
    pub fn is_italic(mut self) -> Self {
        self.font_style.3 = !self.font_style.3;
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
        self.font_style.0 = weight;
        self
    }

    pub fn font_custom(mut self, name: impl ToString) -> Self {
        self.font_style.4 .0 = name.to_string();
        self
    }

    /// Set default font-family default type
    /// Serif
    /// SansSerif | Default
    /// Monospace
    /// Cursive
    /// Fantasy
    /// SystemUi
    pub fn font_default(mut self, def: DefaultFontFamily) -> Self {
        self.font_style.4 .1 = def;
        self
    }

    pub fn font_style(mut self, font: FontStyle) -> Self {
        self.font_style = font;
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

    pub fn get_font(&self) -> &FontStyle {
        &self.font_style
    }

    pub fn get_link(&self) -> &Link {
        &self.link
    }
}
