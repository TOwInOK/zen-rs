use super::SizePremitive;

/// Trait for spacing-related operations.
pub trait Spacing {
    /// Add a gap between elements.
    fn gap(
        self,
        value: SizePremitive,
    ) -> Self;
    /// Add padding around elements.
    fn padding(
        self,
        value: SizePremitive,
    ) -> Self;
}
