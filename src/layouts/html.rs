use crate::components::Components;

mod container;
mod icon;
mod text;

pub use container::*;
pub use icon::*;
pub use text::*;

pub fn html_builder() -> HtmlBuilder {
    HtmlBuilder::default()
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HtmlBuilder {
    /// array of font url imports
    ///
    /// as example:
    /// - @import url('<https://fonts.googleapis.com/css2?family=Doto:wght@100..900&display=swap>');
    css_font_import_urls: String,
    component: Components,
}

// css
impl HtmlBuilder {
    pub fn get_css_font_import_urls(&self) -> &str {
        &self.css_font_import_urls
    }

    pub fn css_font_import_urls(mut self, css_font_import_urls: String) -> Self {
        self.css_font_import_urls = css_font_import_urls;
        self
    }

    pub fn css_disable_default_browser_css(&self) -> &str {
        "* { margin: 0; padding: 0; box-sizing: border-box; overflow: clip; } html, body { height: 100%; line-height: 1.5; } body { background: none; color: inherit; text-align: inherit; } h1, h2, h3, h4, h5, h6 { font-size: inherit; font-weight: inherit; margin: 0; } p { margin: 0; } ul, ol { list-style: none; } a { text-decoration: none; color: inherit; } "
    }

    pub fn build_style(&self) -> String {
        let css_font_import_urls = self.get_css_font_import_urls();
        let disable_default = self.css_disable_default_browser_css();
        format!(r#"<style>{disable_default}{css_font_import_urls}</style>"#)
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

// build
impl HtmlBuilder {
    /// fn for components
    /// Convert components to html
    pub fn render_component(component: &Components) -> String {
        match component {
            Components::Container(component) => container_html(component),
            Components::Text(component) => text_html(component),
            Components::Icon(component) => icon_html(component),
        }
    }
    /// Convert components to html
    pub fn render(&self) -> String {
        let component = self.get_component();
        Self::render_component(component)
    }
    /// Convert components to html with css `<style>`
    pub fn build(&self) -> String {
        let render = self.render();
        let style = self.build_style();
        format!(
            r#"{render}
            {style}"#
        )
    }
    /// Generate default html with style
    pub fn build_as_html(&self) -> String {
        let render = self.render();
        let style = self.build_style();
        format!(r#"<html><head>{style}</head><body>{render}</body></html>"#)
    }
}
