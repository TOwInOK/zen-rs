//! Order/Align aspects types

/// The order of elements in the container relative to the container area
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
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

/// Arrangement of the self in the self container
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
    SpaceBetween,
}
