pub mod github;

use crate::aspects::{Height, Path, Size, StrokeLinecap, StrokeLinejoin, SvgColor, Width};

/// Default attribute for the `xmlns` in SVG elements.
pub static XMLNS: &str = r"http://www.w3.org/2000/svg";

/// Returns a default [Icon] instance.
pub fn icon() -> Icon {
    Icon::default()
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
/// Represents SVG settings, including content, colors, dimensions, and specific attributes.
pub struct Icon {
    /// Content paths of the SVG.
    content: Vec<Path>,
    /// Foreground color of the SVG.
    foreground_color: SvgColor,
    /// Background color of the SVG.
    background_color: SvgColor,
    /// Width of the SVG.
    width: Width,
    /// Height of the SVG.
    height: Height,
    /// Line cap style for strokes.
    stroke_linecap: Option<StrokeLinecap>,
    /// Line join style for strokes.
    stroke_linejoin: Option<StrokeLinejoin>,
    /// Width of the stroke lines.
    stroke_width: Option<f64>,
    /// Viewbox dimensions of the SVG.
    view_box: (u8, u8, u8, u8),
}

impl Icon {
    /// Retrieves the content paths of the SVG.
    pub fn get_content(&self) -> &[String] {
        &(self).content
    }

    /// Retrieves the foreground color of the SVG.
    pub fn get_foreground_color(&self) -> &SvgColor {
        &self.foreground_color
    }

    /// Retrieves the background color of the SVG.
    pub fn get_background_color(&self) -> &SvgColor {
        &self.background_color
    }

    /// Retrieves the width of the SVG.
    pub fn get_width(&self) -> Width {
        self.width
    }

    /// Retrieves the height of the SVG.
    pub fn get_height(&self) -> Height {
        self.height
    }

    /// Adds a new content path to the SVG.
    pub fn content(mut self, content: impl ToString) -> Self {
        self.content.push(content.to_string());
        self
    }

    /// Adds multiple content paths to the SVG.
    pub fn contents(mut self, content: Vec<impl ToString>) -> Self {
        self.content
            .append(&mut content.iter().map(|x| x.to_string()).collect());
        self
    }

    /// Sets the foreground color of the SVG.
    pub fn foreground_color(mut self, foreground_color: impl ToString) -> Self {
        self.foreground_color = SvgColor::Color(foreground_color.to_string());
        self
    }

    /// Removes the foreground color of the SVG.
    pub fn foreground_color_none(mut self) -> Self {
        self.foreground_color = SvgColor::None;
        self
    }

    /// Sets the foreground color to `currentColor`.
    pub fn foreground_color_current_color(mut self) -> Self {
        self.foreground_color = SvgColor::CurrentColor;
        self
    }

    /// Sets the background color of the SVG.
    pub fn background_color(mut self, background_color: impl ToString) -> Self {
        self.background_color = SvgColor::Color(background_color.to_string());
        self
    }

    /// Removes the background color of the SVG.
    pub fn background_color_none(mut self) -> Self {
        self.background_color = SvgColor::None;
        self
    }

    /// Sets the background color to `currentColor`.
    pub fn background_color_current_color(mut self) -> Self {
        self.background_color = SvgColor::CurrentColor;
        self
    }

    /// Sets the width of the SVG.
    pub fn width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the SVG.
    pub fn height(mut self, height: Height) -> Self {
        self.height = height;
        self
    }

    /// Sets both width and height of the SVG.
    pub fn size(mut self, size: Size) -> Self {
        self.width = size;
        self.height = size;
        self
    }

    /// Retrieves the stroke line join style of the SVG.
    pub fn get_stroke_linejoin(&self) -> Option<StrokeLinejoin> {
        self.stroke_linejoin
    }

    /// Retrieves the stroke width of the SVG.
    pub fn get_stroke_width(&self) -> Option<f64> {
        self.stroke_width
    }

    /// Retrieves the viewbox dimensions of the SVG.
    pub fn get_view_box(&self) -> (u8, u8, u8, u8) {
        self.view_box
    }

    /// Retrieves the stroke line cap style of the SVG.
    pub fn get_stroke_linecap(&self) -> Option<StrokeLinecap> {
        self.stroke_linecap
    }

    /// Sets the stroke line join style of the SVG.
    pub fn stroke_linejoin(mut self, stroke_linejoin: StrokeLinejoin) -> Self {
        self.stroke_linejoin = Some(stroke_linejoin);
        self
    }

    /// Sets the stroke width of the SVG.
    pub fn stroke_width(mut self, stroke_width: f64) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }

    /// Sets the viewbox dimensions of the SVG.
    pub fn view_box(mut self, view_box: (u8, u8, u8, u8)) -> Self {
        self.view_box = view_box;
        self
    }

    /// Sets the stroke line cap style of the SVG.
    pub fn stroke_linecap(mut self, stroke_linecap: StrokeLinecap) -> Self {
        self.stroke_linecap = Some(stroke_linecap);
        self
    }
}
