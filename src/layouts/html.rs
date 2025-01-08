use icon::icon_html;
use r#box::box_html;
use text::text_html;

use crate::components::Components;

mod r#box;
mod icon;
mod text;

pub struct HtmlBuilder;
impl HtmlBuilder {
    pub fn render(component: &Components) -> String {
        match component {
            Components::Box(component) => box_html(component),
            Components::Text(component) => text_html(component),
            Components::Icon(component) => icon_html(component),
        }
    }
}
