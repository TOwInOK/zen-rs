/// The direction of elements in the container relative to the container area
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectionPremitive {
    /// T -> B
    #[default]
    TopToBottom,
    /// B -> T
    BottomToTop,
    /// L -> R
    LefToRight,
    /// R -> L
    RightToLeft,
}

/// Set direction
pub trait Direction {
    fn direction(
        self,
        value: DirectionPremitive,
    ) -> Self;
}
