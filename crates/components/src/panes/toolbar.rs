use danubius::components::button::{Button, ButtonSize, ButtonVariant};
use gpui::{Context, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div, px};
use zalmoxis::ActiveTheme;

pub struct Toolbar {
    pub dropdown_open: bool,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            dropdown_open: false,
        }
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Toolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.zalmoxis_colors().clone();
        let this_entity = cx.entity();

        div()
            .flex()
            .items_center()
            .h(px(48.0))
            .px(px(16.0))
            .bg(colors.surface)
            .border_b_1()
            .border_color(colors.outline_variant)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_color(colors.primary)
                    .text_size(px(16.0))
                    .font_weight(FontWeight(700.0))
                    .child("MyApp"),
            )
            .child(div().flex_1())
            .child(
                Button::new("⋮")
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Small)
                    .aria_label("Menu")
                    .on_click({
                        let entity = this_entity.clone();
                        move |_ev, _window, cx| {
                            entity.update(cx, |this, cx| {
                                this.dropdown_open = !this.dropdown_open;
                                cx.notify();
                            });
                        }
                    }),
            )
    }
}
