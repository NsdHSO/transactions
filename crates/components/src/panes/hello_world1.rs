use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Window, div, rgb,
};
use gpui_component::input::{Input, InputEvent, InputState};

pub struct HelloWorld1 {
    input_state: Entity<InputState>,
    value: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl HelloWorld1 {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Type here"));

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    this.value = input_state.read(cx).value().clone();
                    cx.notify();
                }
                _ => {}
            }
        })];

        Self {
            input_state,
            value: SharedString::default(),
            _subscriptions,
        }
    }
}

impl Render for HelloWorld1 {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .child(div().flex().gap_2().child(Input::new(&self.input_state)))
            .child(div().text_color(rgb(0x4a6fa5)).child(self.value.clone()))
    }
}
