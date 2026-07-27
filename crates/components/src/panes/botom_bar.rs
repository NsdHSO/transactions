pub struct BottomBar;
impl Render for BottomBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w_full().h_px(20)
    }
}
