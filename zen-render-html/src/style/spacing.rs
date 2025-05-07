use zen_core::aspect::SizePremitive;

pub fn padding(size: SizePremitive) -> String {
    format!("padding: {}px;", size)
}

pub fn gap(size: SizePremitive) -> String {
    format!("gap: {}px;", size)
}
