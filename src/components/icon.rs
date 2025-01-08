pub mod github;

use crate::aspects::{Height, Path, Size, StrokeLinecap, StrokeLinejoin, SvgColor, Width};

pub static XMLNS: &str = r"http://www.w3.org/2000/svg";

pub fn icon() -> Icon {
    Icon::default()
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
/// Svg
pub struct Icon {
    // content
    content: Vec<Path>,
    // Aspects
    // colors
    foreground_color: SvgColor,
    background_color: SvgColor,
    // size
    width: Width,
    height: Height,
    // Specific svg attributes
    stroke_linecap: Option<StrokeLinecap>,
    stroke_linejoin: Option<StrokeLinejoin>,
    stroke_width: Option<f64>,
    view_box: (u8, u8, u8, u8),
}

impl Icon {
    pub fn get_content(&self) -> &[String] {
        &(self).content
    }

    pub fn get_foreground_color(&self) -> &SvgColor {
        &self.foreground_color
    }

    pub fn get_background_color(&self) -> &SvgColor {
        &self.background_color
    }

    pub fn get_width(&self) -> Width {
        self.width
    }

    pub fn get_height(&self) -> Height {
        self.height
    }

    pub fn content(mut self, content: impl ToString) -> Self {
        self.content.push(content.to_string());
        self
    }

    pub fn contents(mut self, content: Vec<impl ToString>) -> Self {
        self.content
            .append(&mut content.iter().map(|x| x.to_string()).collect());
        self
    }

    pub fn foreground_color(mut self, foreground_color: impl ToString) -> Self {
        self.foreground_color = SvgColor::Color(foreground_color.to_string());
        self
    }
    pub fn foreground_color_none(mut self) -> Self {
        self.foreground_color = SvgColor::None;
        self
    }
    pub fn foreground_color_current_color(mut self) -> Self {
        self.foreground_color = SvgColor::CurrentColor;
        self
    }

    pub fn background_color(mut self, background_color: impl ToString) -> Self {
        self.background_color = SvgColor::Color(background_color.to_string());
        self
    }

    pub fn background_color_none(mut self) -> Self {
        self.background_color = SvgColor::None;
        self
    }
    pub fn background_color_current_color(mut self) -> Self {
        self.background_color = SvgColor::CurrentColor;
        self
    }

    pub fn width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Height) -> Self {
        self.height = height;
        self
    }
    /// set w and h
    pub fn size(mut self, size: Size) -> Self {
        self.width = size;
        self.height = size;
        self
    }

    pub fn get_stroke_linejoin(&self) -> Option<StrokeLinejoin> {
        self.stroke_linejoin
    }

    pub fn get_stroke_width(&self) -> Option<f64> {
        self.stroke_width
    }

    pub fn get_view_box(&self) -> (u8, u8, u8, u8) {
        self.view_box
    }

    pub fn get_stroke_linecap(&self) -> Option<StrokeLinecap> {
        self.stroke_linecap
    }

    pub fn stroke_linejoin(mut self, stroke_linejoin: StrokeLinejoin) -> Self {
        self.stroke_linejoin = Some(stroke_linejoin);
        self
    }

    pub fn stroke_width(mut self, stroke_width: f64) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }

    pub fn view_box(mut self, view_box: (u8, u8, u8, u8)) -> Self {
        self.view_box = view_box;
        self
    }

    pub fn stroke_linecap(mut self, stroke_linecap: StrokeLinecap) -> Self {
        self.stroke_linecap = Some(stroke_linecap);
        self
    }
}
