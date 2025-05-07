use super::RGBA;

/// Set foreground color
pub trait Foreground {
    fn foreground(
        self,
        value: RGBA,
    ) -> Self;
}
