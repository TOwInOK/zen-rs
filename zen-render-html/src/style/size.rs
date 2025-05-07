use zen_core::aspect::SizePremitive;

pub fn size(size: (SizePremitive, SizePremitive)) -> String {
    format!("width: {}; height: {};", size.0, size.1)
}

pub fn font_size(size: SizePremitive) -> String {
    format!("font-size: {};", size)
}
