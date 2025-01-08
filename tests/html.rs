#[cfg(test)]
mod test_html {
    use zen::{
        aspects::DefaultFontFamily,
        components::{
            container::container,
            icon::{icon, Icon},
            text::text,
            Components,
        },
        layouts::html::HtmlBuilder,
    };

    fn gh() -> Icon {
        icon().contents(vec!["M0 0h24v24H0z", "M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"]).stroke_width(2.0).stroke_linecap(zen::aspects::StrokeLinecap::Round).stroke_linejoin(zen::aspects::StrokeLinejoin::Round).view_box((0, 0, 24,24)).size(60).foreground_color_current_color().background_color_none()
    }

    fn layout() -> Components {
        container()
            .component(
                container()
                    .component(
                        container()
                            .component(gh())
                            .components(vec![
                                text()
                                    .content("Gh")
                                    .foreground_color((0, 0, 0, 100))
                                    .size(20)
                                    .font_default(DefaultFontFamily::Monospace),
                                text()
                                    .content("Not ph")
                                    .foreground_color((0, 0, 0, 100))
                                    .size(20)
                                    .font_default(DefaultFontFamily::Monospace),
                            ])
                            .background_color((30, 200, 100, 100))
                            .gap(30)
                            .padding(16)
                            .align_content(zen::aspects::Align::Center)
                            .flex(),
                    )
                    .flex()
                    .border((2, (255, 255, 255, 100), 18)),
            )
            .align_items(zen::aspects::Align::Center)
            .align_content(zen::aspects::Align::Center)
            .width_full()
            .height_full()
            .background_color((0, 0, 0, 100))
            .flex()
            .into()
    }
    #[test]
    fn render() {
        let cp = layout();
        let html = HtmlBuilder::default().component(cp).build_as_html();
        println!("{html}");
        std::fs::write("test_html.html", html).unwrap();
    }
}
