use zen_core::prelude::*;

use crate::size::GetSize;

use super::Node;

#[derive(Debug, Default, Clone)]
/// Container node
pub struct Container {
    // Local content
    /// Nested components
    content: Vec<Node>,

    // Aspects
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
        self.content.push(node.into());
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

impl Padding for Container {
    fn padding(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.padding = value;
        self
    }
}

impl Gap for Container {
    fn gap(
        mut self,
        value: SizePremitive,
    ) -> Self {
        self.gap = value;
        self
    }
}

impl GetSize for Container {
    fn get_size(&self) -> (SizePremitive, SizePremitive) {
        let mut content_size = self.content.iter().fold(
            (SizePremitive::default(), SizePremitive::default()),
            |(mut x, mut y), node| {
                let (width, height) = node.get_size();
                x += width;
                y += height;
                (x, y)
            },
        );
        match self.direction {
            DirectionPremitive::TopToBottom | DirectionPremitive::BottomToTop => {
                content_size.1 += self.gap * (self.content.len() - 1) as f32;
            }
            DirectionPremitive::LefToRight | DirectionPremitive::RightToLeft => {
                content_size.0 += self.gap * (self.content.len() - 1) as f32;
            }
        }
        (
            self.size.0 + self.padding + self.border_size.0 + content_size.0,
            self.size.1 + self.padding + self.border_size.1 + content_size.1,
        )
    }
}
