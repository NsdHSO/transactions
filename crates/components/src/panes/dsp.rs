use crate::divider::ResizeHandle;
use crate::panes::BottomBar;
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb,
};
use zalmoxis::ActiveTheme;

pub struct Dsp {
    bottom_bar: Entity<BottomBar>,
    handle: Entity<ResizeHandle>,
}

impl Dsp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let handle = cx.new(|_| ResizeHandle::vertical(32.0, 32.0, 800.0));

        let _ = cx.observe(&handle, |_, _handle, cx| {
            cx.notify();
        });

        Self {
            bottom_bar: cx.new(|_| BottomBar),
            handle,
        }
    }
}

impl Render for Dsp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.zalmoxis_colors();
        let height = self.handle.read(cx).value;

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
            .child(self.handle.clone())
            .child(div().h(px(height)).child(self.bottom_bar.clone()))
    }
}
