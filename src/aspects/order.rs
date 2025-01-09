//! Order/Align aspects types

// Alignments
/// Order elements in container
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
    SpaceBetween,
}
