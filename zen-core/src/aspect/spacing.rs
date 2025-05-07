use super::SizePremitive;

pub trait Gap {
    /// Add a gap between elements.
    fn gap(
        self,
        value: SizePremitive,
    ) -> Self;
}

pub trait Padding {
    /// Add padding around elements.
    fn padding(
        self,
        value: SizePremitive,
    ) -> Self;
}
