use crate::aspects::{StrokeLinecap, StrokeLinejoin};

use super::{icon, Icon};

/// [Tabler Icon](https://tabler.io/icons)
///
/// Return Icon with Github Otlined settings
pub fn github_outlined() -> Icon {
    icon().contents(vec!["M0 0h24v24H0z", "M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"]).stroke_width(2.0).stroke_linecap(StrokeLinecap::Round).stroke_linejoin(StrokeLinejoin::Round).view_box((0, 0, 24,24)).size(60).foreground_color_current_color().background_color_none()
}
