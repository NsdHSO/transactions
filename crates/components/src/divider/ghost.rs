use gpui::{Context, IntoElement, Pixels, Point, Render, Styled, Window, div, px, rgb};

pub struct DividerGhost {
    pub position: Point<Pixels>,
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

pub struct BottomBarGhost {
    pub y: f32,
}

impl Render for BottomBarGhost {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .h(px(2.0))
            .w_full()
            .bg(rgb(0x00aaff))
            .top(px(self.y))
            .left(px(0.0))
    }
}
