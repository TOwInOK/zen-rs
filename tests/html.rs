#[cfg(test)]
mod test_html {
    use zen_rs::{
        aspects::Align, components::{github::github_outlined, h::mono_text_xl, Components}, layouts::html::HtmlBuilder, vstack
    };

    fn complited_component() -> impl Into<Components> {
        vstack!(
             vstack!(
                 vstack!(
                     github_outlined();
                     mono_text_xl("gh");
                     mono_text_xl("not ph")
                 )
                 .background_color((30, 200, 100, 100))
                 .gap(4)
                 .padding(16)
                 .align_content(zen_rs::aspects::Align::Center)
                 .flex()
             )
             .flex()
             .border((2, (255, 255, 255, 100), 18))
         )
        .flex()
        .width_full()
        .height_full()
        .align_items(Align::Center)
        .align_content(Align::Center)
        .background_color((0, 0, 0, 100))
    }

    #[test]
    fn render() {
        let cp = complited_component();
        let html = HtmlBuilder::default().component(cp).build_as_html();
        println!("{html}");
        std::fs::write("test_html.html", html).unwrap();
    }
}
