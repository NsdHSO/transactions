use danubius::components::input::{Input, InputState, InputVariant};
use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use zalmoxis::ActiveTheme;

pub struct HelloWorld1 {
    input_state: Entity<InputState>,
    input_state2: Entity<InputState>,
}

impl HelloWorld1 {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            input_state: cx.new(|cx| InputState::new(window, cx)),
            input_state2: cx.new(|cx| InputState::new(window, cx)),
        }
    }
}

impl Render for HelloWorld1 {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.zalmoxis_colors();

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_3()
            .p_4()
            .bg(colors.background)
            .child(
                Input::new(&self.input_state)
                    .label("Name")
                    .placeholder("Enter your name")
                    .variant(InputVariant::Outlined)
                    .on_change(|text, _window, _cx| {
                        println!("changed: {}", text);
                    }),
            )
            .child(
                Input::new(&self.input_state2)
                    .label("Name")
                    .placeholder("Enter your name")
                    .variant(InputVariant::Outlined)
                    .on_change(|text, _window, _cx| {
                        println!("changed: {}", text);
                    }),
            )
    }
}
