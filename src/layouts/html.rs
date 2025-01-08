use icon::icon_html;
use r#box::box_html;
use text::text_html;

use crate::components::Components;

mod r#box;
mod icon;
mod text;

pub fn html_builder() -> HtmlBuilder {
    HtmlBuilder::default()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HtmlBuilder {
    /// array of font url imports
    ///
    /// as example:
    /// - @import url('https://fonts.googleapis.com/css2?family=Doto:wght@100..900&display=swap');
    css_font_import_urls: String,
    component: Components,
}

/// css
impl HtmlBuilder {
    pub fn get_css_font_import_urls(&self) -> &str {
        &self.css_font_import_urls
    }

    pub fn css_font_import_urls(mut self, css_font_import_urls: String) -> Self {
        self.css_font_import_urls = css_font_import_urls;
        self
    }

    pub fn build_style(&self) -> String {
        let css_font_import_urls = self.get_css_font_import_urls();
        format!(
            r#"<style>
            {css_font_import_urls}
            </style>"#
        )
    }
}

// set component
impl HtmlBuilder {
    pub fn component(mut self, component: Components) -> Self {
        self.component = component;
        self
    }
    pub fn get_component(&self) -> &Components {
        &self.component
    }
}

/// build
impl HtmlBuilder {
    pub fn render_component(component: &Components) -> String {
        match component {
            Components::Box(component) => box_html(component),
            Components::Text(component) => text_html(component),
            Components::Icon(component) => icon_html(component),
        }
    }
    pub fn render(&self) -> String {
        let component = self.get_component();
        Self::render_component(component)
    }
    pub fn build(&self) -> String {
        let render = self.render();
        let style = self.build_style();
        format!(
            r#"{render}
            {style}"#
        )
    }
}
