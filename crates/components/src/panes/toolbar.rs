use crate::route::Route;
use danubius::components::button::{Button, ButtonSize, ButtonVariant};
use gpui::{Context, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div, px};
use zalmoxis::ActiveTheme;

pub struct Toolbar {
    pub dropdown_open: bool,
    pub current_route: Route,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            dropdown_open: false,
            current_route: Route::Dsp,
        }
    }

    pub fn set_route(&mut self, route: Route, cx: &mut Context<Self>) {
        self.current_route = route;
        cx.notify();
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
        let current = self.current_route;

        let dsp_active = current == Route::Dsp;
        let split_active = current == Route::Split;

        div()
            .flex()
            .items_center()
            .h(px(48.0))
            .px(px(16.0))
            .gap_2()
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
                Button::new("DSP")
                    .variant(if dsp_active {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Ghost
                    })
                    .size(ButtonSize::Small)
                    .on_click({
                        let entity = this_entity.clone();
                        move |_ev, _window, cx| {
                            entity.update(cx, |this, cx| this.set_route(Route::Dsp, cx));
                        }
                    }),
            )
            .child(
                Button::new("Split")
                    .variant(if split_active {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Ghost
                    })
                    .size(ButtonSize::Small)
                    .on_click({
                        let entity = this_entity.clone();
                        move |_ev, _window, cx| {
                            entity.update(cx, |this, cx| this.set_route(Route::Split, cx));
                        }
                    }),
            )
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
