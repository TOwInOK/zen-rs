//! This module defines the `HtmlBuilder` structure and related functions
//! for generating HTML components with embedded CSS styles.
//!
//! The `HtmlBuilder` allows users to build a webpage by specifying various
//! components such as containers, text, and icons, and applying custom CSS
//! styles. It provides methods for importing fonts, disabling default browser
//! CSS, and rendering the components into HTML strings.
//!
//! The components are modular, and the builder pattern is used to allow for
//! easy chaining of method calls to configure and generate the final output.

use crate::components::Components;

mod container;
mod icon;
mod text;

pub use container::*;
pub use icon::*;
pub use text::*;

/// Creates a new `HtmlBuilder` with default values.
pub fn html_builder() -> HtmlBuilder {
    HtmlBuilder::default()
}

/// A builder for generating HTML with embedded CSS.
///
/// This structure allows users to add components (like containers, text, icons)
/// and configure CSS properties such as font imports and default styling.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HtmlBuilder {
    /// Array of font URL imports for the page's CSS.
    ///
    /// Example:
    /// - @import url('<https://fonts.googleapis.com/css2?family=Doto:wght@100..900&display=swap>');
    css_font_import_urls: String,
    /// The main component to be rendered.
    component: Components,
}

// CSS-related methods
impl HtmlBuilder {
    /// Gets the font import URLs for the CSS.
    pub fn get_css_font_import_urls(&self) -> &str {
        &self.css_font_import_urls
    }

    /// Sets the font import URLs for the CSS.
    pub fn css_font_import_urls(mut self, css_font_import_urls: String) -> Self {
        self.css_font_import_urls = css_font_import_urls;
        self
    }

    /// Returns a default CSS reset to disable browser default styling.
    pub fn css_disable_default_browser_css(&self) -> &str {
        "* { margin: 0; padding: 0; box-sizing: border-box; overflow: clip; } html, body { height: 100%; line-height: 1.5; } body { background: none; color: inherit; text-align: inherit; } h1, h2, h3, h4, h5, h6 { font-size: inherit; font-weight: inherit; margin: 0; } p { margin: 0; } ul, ol { list-style: none; } a { text-decoration: none; color: inherit; } "
    }

    /// Builds the full CSS style, including any custom font imports.
    pub fn build_style(&self) -> String {
        let css_font_import_urls = self.get_css_font_import_urls();
        let disable_default = self.css_disable_default_browser_css();
        format!(r#"<style>{disable_default}{css_font_import_urls}</style>"#)
    }
}

// Component-related methods
impl HtmlBuilder {
    /// Sets the component to be rendered.
    pub fn component(mut self, component: Components) -> Self {
        self.component = component;
        self
    }

    /// Gets the current component to be rendered.
    pub fn get_component(&self) -> &Components {
        &self.component
    }
}

// Methods for building HTML output
impl HtmlBuilder {
    /// Converts a given component to HTML.
    ///
    /// This function matches the component type and calls the respective
    /// rendering function for `Container`, `Text`, or `Icon` components.
    pub fn render_component(component: &Components) -> String {
        match component {
            Components::Container(component) => container_html(component),
            Components::Text(component) => text_html(component),
            Components::Icon(component) => icon_html(component),
        }
    }

    /// Renders the current component to HTML.
    pub fn render(&self) -> String {
        let component = self.get_component();
        Self::render_component(component)
    }

    /// Renders the current component along with the CSS styles as a complete HTML.
    pub fn build(&self) -> String {
        let render = self.render();
        let style = self.build_style();
        format!(
            r#"{render}
            {style}"#
        )
    }

    /// Generates a complete HTML document, including the head and body sections.
    pub fn build_as_html(&self) -> String {
        let render = self.render();
        let style = self.build_style();
        format!(r#"<html><head>{style}</head><body>{render}</body></html>"#)
    }
}
