#[cfg(test)]
mod tests {
    use std::fs;

    use zen_core::prelude::*;
    use zen_render_html::Html;
    use zen_tree::{
        node::{Node, text::text},
        prelude::vstack,
    };

    fn component() -> Node {
        vstack((text()
            .content("hello world!")
            .foreground((100.0, 100.0, 100.0, 100.0))
            .size(16.0)
            .padding(16.0)
            .background((0.0, 255.0, 255.0, 255.0)),))
        .alignment(AlignmentPrimitive::Centerc)
        .into()
    }

    #[test]
    fn render() {
        let file = Html::new(component()).render().unwrap();
        fs::write("test.html", file).unwrap();
    }
}
