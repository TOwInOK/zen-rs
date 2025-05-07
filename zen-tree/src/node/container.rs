use zen_core::{
    aspect::{RGBA, SizePremitive},
    prelude::{
        Alignment, AlignmentPrimitive, Background, Border, BorderParts, Direction,
        DirectionPremitive, Foreground, Size, Spacing,
    },
};

use super::Node;

#[derive(Debug, Default, Clone)]
/// Container node
pub struct Container {
    /// Nested components
    components: Vec<Node>,
    /// Background color
    bg: RGBA,
    /// Width & Height
    size: (SizePremitive, SizePremitive),
    /// Gap between components
    gap: SizePremitive,
    /// Padding around components
    padding: SizePremitive,
    /// Direction of components
    direction: DirectionPremitive,
    /// Alignment of components
    alignment: AlignmentPrimitive,
    /// Border size
    border_size: BorderParts,
    /// Border radius
    border_radius: BorderParts,
    /// Border color
    border_color: RGBA,
}

pub fn container() -> Container {
    Container::default()
}

impl Container {
    pub fn push(
        &mut self,
        node: impl Into<Node>,
    ) {
        self.components.push(node.into());
    }
}

impl Direction for Container {
    fn direction(
        mut self,
        value: DirectionPremitive,
    ) -> Self {
        self.direction = value;
        self
    }
}

impl Alignment for Container {
    fn alignment(
        mut self,
        value: AlignmentPrimitive,
    ) -> Self {
        self.alignment = value;
        self
    }
}

impl Background for Container {
    fn background(
        mut self,
        value: RGBA,
    ) -> Self {
        self.bg = value;
        self
    }
}

impl Foreground for Container {
    /// Set border color
    fn foreground(
        mut self,
        value: RGBA,
    ) -> Self {
        self.border_color = value;
        self
    }
}

impl Border for Container {
    fn border_color(
        mut self,
        value: RGBA,
    ) -> Self {
        self.border_color = value;
        self
    }

    fn border_radius_parts(
        mut self,
        value: BorderParts,
    ) -> Self {
        self.border_radius = value;
        self
    }

    fn border_size_parts(
        mut self,
        value: BorderParts,
    ) -> Self {
        self.border_size = value;
        self
    }
}

impl Size for Container {
    fn width(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.size.0 = value;
        self
    }

    fn height(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.size.1 = value;
        self
    }
}

impl Spacing for Container {
    fn gap(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.gap = value;
        self
    }

    fn padding(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.padding = value;
        self
    }
}
