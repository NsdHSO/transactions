use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px};
use zalmoxis::ActiveTheme;

pub struct LeftBar;

impl Render for LeftBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.zalmoxis_colors();

        div()
            .flex()
            .items_center()
            .h_full()
            .w_9()
            .bg(colors.surface)
            .border_l_1()
            .border_color(colors.outline_variant)
            .text_color(colors.on_surface_variant)
            .text_size(px(12.0))
            .child("Left Bar")
    }
}
