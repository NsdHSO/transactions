use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window, div, rgb,
};

pub struct HelloWorld1 {
    pub text: SharedString,
    pub input: Entity<InputField>,
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
