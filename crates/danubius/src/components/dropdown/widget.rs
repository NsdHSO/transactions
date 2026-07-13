use std::rc::Rc;

use gpui::{
    App, ClickEvent, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use zalmoxis::ActiveTheme;

use super::super::button::{Button, ButtonVariant};

type ActivateCb = Box<dyn Fn(&mut Window, &mut App)>;

pub struct MenuEntry {
    pub label: SharedString,
    on_activate: ActivateCb,
}

impl MenuEntry {
    pub fn new(
        label: impl Into<SharedString>,
        on_activate: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            on_activate: Box::new(on_activate),
        }
    }
}

#[derive(IntoElement)]
pub struct Dropdown {
    entries: Vec<MenuEntry>,
}

impl Dropdown {
    pub fn new(entries: Vec<MenuEntry>) -> Self {
        Self { entries }
    }
}

impl RenderOnce for Dropdown {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.zalmoxis_colors().clone();

        div()
            .w(px(200.0))
            .py(px(4.0))
            .bg(colors.surface)
            .border_1()
            .border_color(colors.outline)
            .rounded(px(8.0))
            .shadow_lg()
            .children(self.entries.into_iter().map(|entry| {
                let cb = Rc::new(entry.on_activate);
                Button::new(entry.label)
                    .variant(ButtonVariant::Ghost)
                    .full_width(true)
                    .on_click({
                        let cb = Rc::clone(&cb);
                        move |_ev: &ClickEvent, w: &mut Window, cx: &mut App| cb(w, cx)
                    })
                    .into_element()
            }))
    }
}
