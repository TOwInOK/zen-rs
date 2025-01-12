//! Container component

pub mod stack;

use crate::aspects::{Align, BackgroundColor, BorderPart, BorderStyle, Color, Gap, Height, Order, Padding, Size, Width};

use super::Components;

/// Returns a default [Container] instance.
pub fn container() -> Container {
    Container::default()
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
/// Represents a container with customizable aspects like size, color, border, and alignment.
pub struct Container {
    /// Components contained within the container.
    components: Vec<Components>,
    /// Background color of the container.
    background_color: BackgroundColor,
    /// Width of the container.
    width: Width,
    /// Whether the container should occupy the full width.
    is_width_full: bool,
    /// Height of the container.
    height: Height,
    /// Whether the container should occupy the full height.
    is_height_full: bool,
    /// Border properties of the container.
    border: BorderStyle,
    /// Layout direction of components within the container.
    direction: Order,
    /// Gap between components in the container.
    gap: Gap,
    /// Padding inside the container.
    padding: Padding,
    /// Whether the container uses flexible layout.
    is_flex: bool,
    /// Alignment of content within the container.
    align_content: Align,
    /// Alignment of individual items within the container.
    align_items: Align,
}

impl Container {
    /// Adds a single component to the container.
    pub fn component(mut self, component: impl Into<Components>) -> Self {
        self.components.push(component.into());
        self
    }

    /// Adds multiple components to the container.
    /// **note** at once can be added only 1 type of object
    pub fn components(
        mut self,
        components: impl IntoIterator<Item = impl Into<Components>>,
    ) -> Self {
        self.components
            .append(&mut components.into_iter().map(|x| x.into()).collect());
        self
    }

    /// Retrieves all components in the container.
    pub fn get_components(&self) -> &[Components] {
        &self.components
    }

    /// Sets the gap between components in the container.
    pub fn gap(mut self, gap: Gap) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the padding inside the container.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Retrieves the gap setting of the container.
    pub fn get_gap(&self) -> &Gap {
        &self.gap
    }

    /// Retrieves the padding setting of the container.
    pub fn get_padding(&self) -> &Padding {
        &self.padding
    }

    /// Sets the background color of the container.
    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = background_color;
        self
    }

    /// Sets the width of the container.
    pub fn width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the container.
    pub fn height(mut self, height: Height) -> Self {
        self.height = height;
        self
    }

    /// Toggles whether the container occupies the full width.
    pub fn width_full(mut self) -> Self {
        self.is_width_full = !self.is_width_full;
        self
    }

    /// Toggles whether the container occupies the full height.
    pub fn height_full(mut self) -> Self {
        self.is_height_full = !self.is_height_full;
        self
    }

    /// Sets the border properties of the container.
    pub fn border(mut self, border: BorderStyle) -> Self {
        self.border = border;
        self
    }

    /// Sets the border color properties of the container.
    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border.1 = border_color;
        self
    }

    /// Set the border radius propertie of the container.
    pub fn border_radius(mut self, border_size: Size) -> Self {
        self.border.2 = border_size;
        self
    }

    /// Sets the border properties of the container.
    pub fn border_size(mut self, border_size: BorderPart) -> Self {
        self.border.0 = border_size;
        self
    }

    /// Sets the border properties size of the container.
    pub fn border_size_full(mut self, border_size_full: Size) -> Self {
        self.border.0 = (border_size_full,border_size_full,border_size_full,border_size_full);
        self
    }

    /// Set the border size propertie
    pub fn border_size_l(mut self, border_size_l: Size) -> Self {
        let (_,t,b,r) = self.border.0;
        self.border.0 = (border_size_l, t, b ,r);
        self
    }

    /// Set the border size propertie
    pub fn border_size_t(mut self, border_size_t: Size) -> Self {
        let (l,_,b,r) = self.border.0;
        self.border.0 = (l, border_size_t, b ,r);
        self
    }

    /// Set the border size propertie
    pub fn border_size_b(mut self, border_size_b: Size) -> Self {
        let (l,t,_,r) = self.border.0;
        self.border.0 = (l, t, border_size_b ,r);
        self
    }


    /// Set the border size propertie
    pub fn border_size_r(mut self, border_size_r: Size) -> Self {
        let (l,t,b,_) = self.border.0;
        self.border.0 = (l, t, b ,border_size_r);
        self
    }


    /// Sets the layout direction of components in the container.
    pub fn direction(mut self, direction: Order) -> Self {
        self.direction = direction;
        self
    }

    /// Toggles whether the container uses flexible layout.
    pub fn flex(mut self) -> Self {
        self.is_flex = !self.is_flex;
        self
    }

    /// Sets the alignment of content within the container.
    pub fn align_content(mut self, align_content: Align) -> Self {
        self.align_content = align_content;
        self
    }

    /// Sets the alignment of individual items within the container.
    pub fn align_items(mut self, align_items: Align) -> Self {
        self.align_items = align_items;
        self
    }

    /// Retrieves the flexible layout setting of the container.
    pub fn get_flex(&self) -> &bool {
        &self.is_flex
    }

    /// Retrieves the full-width setting of the container.
    pub fn get_width_full(&self) -> &bool {
        &self.is_width_full
    }

    /// Retrieves the full-height setting of the container.
    pub fn get_height_full(&self) -> &bool {
        &self.is_height_full
    }

    /// Retrieves the alignment of content within the container.
    pub fn get_align_content(&self) -> &Align {
        &self.align_content
    }

    /// Retrieves the alignment of items within the container.
    pub fn get_align_items(&self) -> &Align {
        &self.align_items
    }

    /// Retrieves the background color of the container.
    pub fn get_background_color(&self) -> &(u8, u8, u8, u8) {
        &self.background_color
    }

    /// Retrieves the width of the container.
    pub fn get_width(&self) -> &u64 {
        &self.width
    }

    /// Retrieves the height of the container.
    pub fn get_height(&self) -> &u64 {
        &self.height
    }

    /// Retrieves the border properties of the container.
    pub fn get_border(&self) -> &BorderStyle {
        &self.border
    }

    /// Retrieves the layout direction of the container.
    pub fn get_direction(&self) -> &Order {
        &self.direction
    }
}
