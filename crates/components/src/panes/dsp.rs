use crate::panes::BottomBar;
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, rgb,
};
use zalmoxis::ActiveTheme;

pub struct Dsp {
    bottom_bar: Entity<BottomBar>,
}

impl Dsp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            bottom_bar: cx.new(|_| BottomBar),
        }
    }
}

impl Render for Dsp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let colors = _cx.zalmoxis_colors();

        div()
            .flex()
            .bg(colors.background)
            .flex_col()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child("DSP Screen"),
            )
            .child(self.bottom_bar.clone())
    }
}
