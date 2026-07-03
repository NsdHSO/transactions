use gpui::{
    App, AppContext, Application, Bounds, Context, DragMoveEvent, Entity, InteractiveElement,
    IntoElement, ParentElement, Pixels, Point, Render, SharedString, Styled, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

struct HelloWorld1 {
    text: SharedString,
}

impl Render for HelloWorld1 {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x3a3a3a))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

#[derive(Clone, Copy)]
struct DividerDrag;

struct DividerGhost {
    position: Point<Pixels>,
}

impl Render for DividerGhost {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .w(px(2.0))
            .h_full()
            .bg(rgb(0x00aaff))
            .left(self.position.x)
            .top(self.position.y)
    }
}

struct Split {
    left: Entity<HelloWorld>,
    right: Entity<HelloWorld1>,
    left_width: f32,
}

const DIVIDER_WIDTH: f32 = 4.0;
const MIN_PANE: f32 = 100.0;

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

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let left = cx.new(|_| HelloWorld {
                    text: "World".into(),
                });
                let right = cx.new(|_| HelloWorld1 {
                    text: "World1".into(),
                });
                cx.new(|_| Split {
                    left,
                    right,
                    left_width: 500.0,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
