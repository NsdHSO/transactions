use components::panes::{HelloWorld, HelloWorld1};
use components::{DIVIDER_WIDTH, DividerDrag, DividerGhost, MIN_PANE};
use gpui::{
    Context, DragMoveEvent, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled,
    Window, div, prelude::*, px, rgb,
};

pub struct Split {
    pub left: Entity<HelloWorld>,
    pub right: Entity<HelloWorld1>,
    pub left_width: f32,
}

impl Render for Split {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .child(
                div()
                    .h_full()
                    .w(px(self.left_width))
                    .child(self.left.clone()),
            )
            .child(
                div()
                    .id("divider")
                    .h_full()
                    .w(px(DIVIDER_WIDTH))
                    .bg(rgb(0x808080))
                    .hover(|s| s.bg(rgb(0x00aaff)))
                    .cursor_ew_resize()
                    .on_drag(DividerDrag, |_, pos, _, cx| {
                        cx.new(|_| DividerGhost { position: pos })
                    })
                    .on_drag_move(cx.listener(
                        |this, e: &DragMoveEvent<DividerDrag>, window, cx| {
                            let total = f32::from(window.viewport_size().width);
                            let max = (total - MIN_PANE).max(MIN_PANE);
                            this.left_width = f32::from(e.event.position.x).clamp(MIN_PANE, max);
                            cx.notify();
                        },
                    )),
            )
            .child(div().flex_1().h_full().child(self.right.clone()))
    }
}
