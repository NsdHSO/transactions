use std::rc::Rc;

use gpui::{
    App, ClickEvent, FontWeight, InteractiveElement, IntoElement, KeyDownEvent, ParentElement as _,
    RenderOnce, Role, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};
use zalmoxis::ActiveTheme;

use super::style::{ButtonSize, ButtonVariant};

type ClickCb = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

const TRANSPARENT: gpui::Hsla = gpui::Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.0,
    a: 0.0,
};

#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    disabled: bool,
    variant: ButtonVariant,
    size: ButtonSize,
    full_width: bool,
    aria_label: Option<SharedString>,
    on_click: Option<ClickCb>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            full_width: false,
            aria_label: None,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.zalmoxis_colors().clone();
        let label = self.label.clone();

        let (bg, text_color, hover_bg) = if self.disabled {
            (
                colors.surface_variant,
                colors.on_surface.opacity(0.38),
                colors.surface_variant,
            )
        } else {
            match self.variant {
                ButtonVariant::Primary => (
                    colors.primary,
                    colors.on_primary,
                    colors.primary.opacity(0.9),
                ),
                ButtonVariant::Secondary => (
                    colors.secondary_container,
                    colors.on_secondary_container,
                    colors.secondary_container.opacity(0.8),
                ),
                ButtonVariant::Outline => {
                    (TRANSPARENT, colors.primary, colors.primary.opacity(0.08))
                }
                ButtonVariant::Ghost => (TRANSPARENT, colors.primary, colors.primary.opacity(0.08)),
                ButtonVariant::Danger => (colors.error, colors.on_error, colors.error.opacity(0.9)),
            }
        };

        let height = self.size.height_px();
        let font_size = self.size.font_size_px();
        let padding_x = self.size.padding_x_px();

        let border_color = if self.disabled {
            TRANSPARENT
        } else if self.variant == ButtonVariant::Outline {
            colors.outline
        } else {
            TRANSPARENT
        };

        let a11y_label = self.aria_label.clone().unwrap_or_else(|| label.clone());
        let on_click = self.on_click;

        let mut btn = div()
            .id(label.clone())
            .flex()
            .items_center()
            .justify_center()
            .h(px(height))
            .px(px(padding_x))
            .rounded(px(6.0))
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .text_size(px(font_size))
            .text_color(text_color)
            .font_weight(FontWeight(500.0))
            .cursor_pointer()
            .tab_index(0)
            .role(Role::Button)
            .aria_label(a11y_label)
            .hover(move |style| style.bg(hover_bg));

        if let Some(cb) = on_click {
            let cb = Rc::new(cb);
            let cb_click = Rc::clone(&cb);
            btn = btn
                .on_click(move |ev, w, cx| cb_click(ev, w, cx))
                .on_key_down(move |ev: &KeyDownEvent, w, cx| {
                    if ev.keystroke.key == "enter" || ev.keystroke.key == "space" {
                        cb(&ClickEvent::default(), w, cx);
                    }
                });
        }

        if self.full_width {
            btn = btn.w_full();
        }

        btn.child(label)
    }
}
