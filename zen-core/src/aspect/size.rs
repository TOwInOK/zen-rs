pub use super::SizePremitive as S;

/// Trait for size-related operations.
pub trait Size {
    /// Set width and height
    fn size(
        self,
        value: S,
    ) -> Self
    where
        Self: Sized,
    {
        self.width(value).height(value)
    }
    /// Set width
    fn width(
        self,
        value: S,
    ) -> Self;
    /// Set height
    fn height(
        self,
        value: S,
    ) -> Self;
}
