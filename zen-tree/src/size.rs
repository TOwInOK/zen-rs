use zen_core::aspect::SizePremitive;

pub trait GetSize {
    /// Return the size of the component
    fn get_size(&self) -> (SizePremitive, SizePremitive);
}
