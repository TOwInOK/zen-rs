/// Arrangement of the self in the self container
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlignmentPrimitive {
    #[default]
    Left,
    Center,
    Right,
    SpaceBetween,
}

/// Set alignment
pub trait Alignment {
    fn alignment(
        self,
        value: AlignmentPrimitive,
    ) -> Self;
}
