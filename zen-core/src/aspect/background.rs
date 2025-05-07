use super::RGBA;

/// Set background color
pub trait Background {
    fn background(
        self,
        value: RGBA,
    ) -> Self;
}
