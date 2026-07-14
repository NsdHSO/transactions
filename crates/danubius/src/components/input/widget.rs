use gpui::{
    App, ClickEvent, ClipboardItem, Entity, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};
use zalmoxis::ActiveTheme;

use super::state::InputState;
use super::style::{InputSize, InputVariant};

type ChangeCb = Box<dyn Fn(&SharedString, &mut Window, &mut App)>;
type FocusCb = Box<dyn Fn(bool, &mut Window, &mut App)>;
type ClickCb = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Input {
    state: Entity<InputState>,
    variant: InputVariant,
    size: InputSize,
    label: Option<SharedString>,
    error: Option<SharedString>,
    prefix: Option<gpui::AnyElement>,
    suffix: Option<gpui::AnyElement>,
    disabled: bool,
    placeholder: Option<SharedString>,
    on_change: Option<ChangeCb>,
    on_focus: Option<FocusCb>,
    on_click: Option<ClickCb>,
}

impl Input {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            variant: InputVariant::Outlined,
            size: InputSize::Medium,
            label: None,
            error: None,
            prefix: None,
            suffix: None,
            disabled: false,
            placeholder: None,
            on_change: None,
            on_focus: None,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn error(mut self, error: impl Into<SharedString>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn prefix(mut self, element: impl IntoElement) -> Self {
        self.prefix = Some(element.into_any_element());
        self
    }

    pub fn suffix(mut self, element: impl IntoElement) -> Self {
        self.suffix = Some(element.into_any_element());
        self
    }

    pub fn on_change(mut self, f: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn on_focus(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_focus = Some(Box::new(f));
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.zalmoxis_colors().clone();

        let is_focused = self.state.read(cx).focus_handle().is_focused(window);
        let has_error = self.error.is_some();

        let (bg, border, text_color, placeholder_color, label_color) = if self.disabled {
            (
                colors.surface_variant,
                colors.outline_variant,
                colors.on_surface.opacity(0.38),
                colors.on_surface.opacity(0.38),
                colors.on_surface.opacity(0.38),
            )
        } else if has_error {
            (
                colors.surface,
                colors.error,
                colors.on_surface,
                colors.on_surface_variant,
                colors.error,
            )
        } else {
            let active_border = if is_focused {
                colors.primary
            } else {
                colors.outline
            };
            (
                colors.surface,
                active_border,
                colors.on_surface,
                colors.on_surface_variant,
                colors.primary,
            )
        };

        self.state.update(cx, |state, _| {
            state.set_disabled(self.disabled);
            if let Some(text) = &self.placeholder {
                state.set_placeholder(text.clone());
            }
            state.set_text_colors(text_color, placeholder_color);
            state.set_selection_background(colors.primary.opacity(0.3));
        });

        let height = self.size.height_px();
        let font_size = self.size.font_size_px();
        let padding_x = self.size.padding_x_px();

        let on_change = self.on_change;
        let on_click_cb = self.on_click;
        let state_entity = self.state.clone();

        let key_handler = {
            let state = state_entity.clone();
            let on_change_inner = on_change;
            move |keystroke: &gpui::KeyDownEvent, window: &mut Window, cx: &mut App| {
                let k = &keystroke.keystroke;
                let mut changed: bool = false;
                state.update(cx, |s, cx| {
                    let mut redraw = false;
                    if k.modifiers.platform {
                        match k.key.as_str() {
                            "a" => {
                                s.select_all();
                                redraw = true;
                            }
                            "c" => {
                                let text = s.value().to_string();
                                if !text.is_empty() {
                                    cx.write_to_clipboard(ClipboardItem::new_string(text));
                                }
                            }
                            "x" => {
                                let text = s.value().to_string();
                                if !text.is_empty() {
                                    cx.write_to_clipboard(ClipboardItem::new_string(text));
                                    s.set_value(SharedString::default());
                                    changed = true;
                                }
                            }
                            "v" => {
                                if let Some(item) = cx.read_from_clipboard() {
                                    for entry in item.entries {
                                        if let gpui::ClipboardEntry::String(entry) = entry {
                                            s.insert_text(&entry.text, cx);
                                            changed = true;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        if redraw {
                            cx.notify();
                        }
                        return;
                    }

                    match k.key.as_str() {
                        "backspace" => {
                            s.backspace(cx);
                            changed = true;
                        }
                        "delete" => {
                            s.delete(cx);
                            changed = true;
                        }
                        "left" => {
                            s.move_left();
                            redraw = true
                        }
                        "right" => {
                            s.move_right();
                            redraw = true
                        }
                        "home" => {
                            s.move_home();
                            redraw = true
                        }
                        "end" => {
                            s.move_end();
                            redraw = true
                        }
                        _ => {
                            if let Some(ch) = &k.key_char {
                                s.insert_text(ch, cx);
                                changed = true;
                            }
                        }
                    }
                    if redraw {
                        cx.notify();
                    }
                });

                if changed {
                    let new = state.read(cx).value().clone();
                    if let Some(ref cb) = on_change_inner {
                        cb(&new, window, cx);
                    }
                }
            }
        };

        let input_body = div()
            .id(("input-field", state_entity.entity_id()))
            .flex()
            .flex_row()
            .items_center()
            .h(px(height))
            .w_full()
            .px(px(padding_x))
            .bg(bg)
            .text_size(px(font_size))
            .child(state_entity.clone());

        let input_body = if let Some(prefix) = self.prefix {
            input_body.child(prefix)
        } else {
            input_body
        };

        let input_body = if let Some(suffix) = self.suffix {
            input_body.child(suffix)
        } else {
            input_body
        };

        let input_body = match self.variant {
            InputVariant::Outlined => input_body.rounded(px(4.0)).border_1().border_color(border),
            InputVariant::Filled => input_body
                .rounded_tl(px(4.0))
                .rounded_tr(px(4.0))
                .border_b_1()
                .border_color(border),
            InputVariant::Standard => input_body.border_b_1().border_color(border),
        };

        let input_body = input_body
            .key_context("Input")
            .track_focus(state_entity.read(cx).focus_handle())
            .tab_index(0)
            .hover(|style| style.border_color(colors.on_surface))
            .on_key_down(key_handler);

        let input_body = if let Some(cb) = on_click_cb {
            input_body.on_click(move |ev, w, cx| cb(ev, w, cx))
        } else {
            input_body
        };

        let mut container = div().flex().flex_col().gap(px(4.0));

        if let Some(label) = self.label {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .text_color(label_color)
                    .mb(px(2.0))
                    .child(label),
            );
        }

        container = container.child(input_body);

        if let Some(error) = self.error {
            container = container.child(
                div()
                    .text_size(px(11.0))
                    .text_color(colors.error)
                    .mt(px(2.0))
                    .child(error),
            );
        }

        container
    }
}
