use crate::aspects::{BackgroundColor, Border, ForegroundColor, Height, Order, Width};

use super::Components;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Box {
    // components
    components: Vec<Components>,
    // Aspects
    foreground_color: ForegroundColor,
    background_color: BackgroundColor,
    width: Width,
    height: Height,
    border: Border,
    direction: Order,
}

pub fn r#box() -> Box {
    Box::default()
}

impl Box {
    /// Pust component to box
    pub fn component(&mut self, component: Components) {
        self.components.push(component);
    }

    pub fn foreground_color(mut self, foreground_color: ForegroundColor) -> Self {
        self.foreground_color = foreground_color;
        self
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

    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn direction(mut self, direction: Order) -> Self {
        self.direction = direction;
        self
    }

    pub fn get_foreground_color(&self) -> &(u8, u8, u8, u8) {
        &self.foreground_color
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

    pub fn get_border(&self) -> &(bool, u64) {
        &self.border
    }

    pub fn get_direction(&self) -> &Order {
        &self.direction
    }
}
