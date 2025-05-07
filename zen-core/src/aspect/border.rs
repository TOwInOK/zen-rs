use super::{RGBA, SizePremitive};
pub type BorderParts = (SizePremitive, SizePremitive, SizePremitive, SizePremitive);

/// Set border style
pub trait Border {
    /// Set border color
    fn border_color(
        self,
        value: RGBA,
    ) -> Self;
    /// Set border radius
    fn border_radius(
        self,
        value: SizePremitive,
    ) -> Self
    where
        Self: Sized,
    {
        self.border_radius_parts((value, value, value, value))
    }
    /// Set border radius parts
    fn border_radius_parts(
        self,
        value: BorderParts,
    ) -> Self;
    /// Set border size
    fn border_size(
        self,
        value: SizePremitive,
    ) -> Self
    where
        Self: Sized,
    {
        self.border_size_parts((value, value, value, value))
    }
    /// Set border size parts
    fn border_size_parts(
        self,
        value: BorderParts,
    ) -> Self;
    /// Set border
    fn border(
        self,
        color: RGBA,
        radius: SizePremitive,
        size: SizePremitive,
    ) -> Self
    where
        Self: Sized,
    {
        self.border_parts(
            color,
            (radius, radius, radius, radius),
            (size, size, size, size),
        )
    }
    /// Set border parts
    fn border_parts(
        self,
        color: RGBA,
        radius: BorderParts,
        size: BorderParts,
    ) -> Self
    where
        Self: Sized,
    {
        self.border_color(color)
            .border_radius_parts(radius)
            .border_size_parts(size)
    }
}
