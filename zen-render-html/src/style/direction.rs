use zen_core::prelude::DirectionPremitive;

pub fn direction(attr: DirectionPremitive) -> String {
    format!(
        "display: flex; flex-direction: {};",
        match attr {
            DirectionPremitive::TopToBottom => "column",
            DirectionPremitive::BottomToTop => "column-reverse",
            DirectionPremitive::LefToRight => "row",
            DirectionPremitive::RightToLeft => "row-reverse",
        }
    )
}
