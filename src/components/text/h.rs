//! Components with fixed size
//! [Tailwindcss font size](https://tailwindcss.com/docs/font-size)

macro_rules! h {
    ($name:tt, $font_size:expr) => {
        paste::paste! {
            #[doc = stringify!(text with $fint_size px size and dark color font)]
            pub fn [<text_$name>](content: impl ToString) -> crate::components::Text {
                crate::components::text()
                    .content(content)
                    .size($font_size)
                    .foreground_color((0, 0, 0, 100))
            }
        }
        paste::paste! {
            #[doc = stringify!(mono text with $fint_size px size and dark color font)]
            pub fn [<mono_text_$name>](content: impl ToString) -> crate::components::Text {
                crate::components::text()
                    .content(content)
                    .size($font_size)
                    .foreground_color((0, 0, 0, 100))
                    .font_default(crate::aspects::DefaultFontFamily::Monospace)
            }
        }
    };
}

h!(9xl, 128);
h!(8xl, 96);
h!(7xl, 72);
h!(6xl, 60);
h!(5xl, 48);
h!(4xl, 36);
h!(3xl, 30);
h!(2xl, 24);
h!(xl, 20);
h!(lg, 18);
h!(base, 16);
h!(sm, 14);
h!(xs, 12);
