use zen_core::aspect::SizePremitive;
use zen_tree::size::GetSize;

pub fn size<T: GetSize>(size: &T) -> String {
    let size = size.get_size();
    format!("width: {}px; height: {}px;", size.0, size.1)
}

pub fn font_size(size: SizePremitive) -> String {
    format!("font-size: {}px;", size)
}
