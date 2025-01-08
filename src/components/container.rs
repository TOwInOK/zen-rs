use crate::aspects::{Align, BackgroundColor, Border, Gap, Height, Order, Padding, Width};

use super::Components;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Container {
    // components
    components: Vec<Components>,
    // Aspects
    // color
    background_color: BackgroundColor,
    // size
    width: Width,
    is_width_full: bool,
    height: Height,
    is_height_full: bool,
    // border
    border: Border,
    // order
    direction: Order,
    // spaces
    gap: Gap,
    padding: Padding,
    // flex
    is_flex: bool,
    // Align
    // Align Ordering/Spacing of components
    align_content: Align,
    // Align self position of components
    align_items: Align,
}

pub fn container() -> Container {
    Container::default()
}

impl Container {
    /// Pust component to box
    pub fn component(mut self, component: impl Into<Components>) -> Self {
        self.components.push(component.into());
        self
    }

    pub fn components(mut self, components: Vec<impl Into<Components>>) -> Self {
        self.components
            .append(&mut components.into_iter().map(|x| x.into()).collect());
        self
    }

    pub fn get_components(&self) -> &[Components] {
        &self.components
    }

    pub fn gap(mut self, gap: Gap) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn get_gap(&self) -> &Gap {
        &self.gap
    }
    pub fn get_padding(&self) -> &Padding {
        &self.padding
    }

    pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
        self.background_color = background_color;
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

    pub fn width_full(mut self) -> Self {
        self.is_width_full = !self.is_width_full;
        self
    }

    pub fn height_full(mut self) -> Self {
        self.is_height_full = !self.is_height_full;
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn direction(mut self, direction: Order) -> Self {
        self.direction = direction;
        self
    }

    pub fn flex(mut self) -> Self {
        self.is_flex = !self.is_flex;
        self
    }
    pub fn align_content(mut self, align_content: Align) -> Self {
        self.align_content = align_content;
        self
    }
    pub fn align_items(mut self, align_items: Align) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn get_flex(&self) -> &bool {
        &self.is_flex
    }

    pub fn get_width_full(&self) -> &bool {
        &self.is_width_full
    }

    pub fn get_height_full(&self) -> &bool {
        &self.is_height_full
    }

    pub fn get_align_content(&self) -> &Align {
        &self.align_content
    }

    pub fn get_align_items(&self) -> &Align {
        &self.align_items
    }

    pub fn get_background_color(&self) -> &(u8, u8, u8, u8) {
        &self.background_color
    }

    pub fn get_width(&self) -> &u64 {
        &self.width
    }

    pub fn get_height(&self) -> &u64 {
        &self.height
    }

    pub fn get_border(&self) -> &(u64, (u8, u8, u8, u8), u64) {
        &self.border
    }

    pub fn get_direction(&self) -> &Order {
        &self.direction
    }
}
