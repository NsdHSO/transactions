use std::ops::Range;

use gpui::{
    App, Context, EventEmitter, FocusHandle, Focusable, Hsla, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, div,
};

#[derive(Clone)]
pub enum InputEvent {
    Change,
    Focus,
    Blur,
}

pub struct InputState {
    value: SharedString,
    cursor_pos: usize,
    selection: Option<Range<usize>>,
    focus_handle: FocusHandle,
    placeholder: SharedString,
    disabled: bool,
    text_color: Hsla,
    placeholder_color: Hsla,
    selection_background: Hsla,
}

impl EventEmitter<InputEvent> for InputState {}

impl InputState {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            value: SharedString::default(),
            cursor_pos: 0,
            selection: None,
            focus_handle: cx.focus_handle().tab_stop(true),
            selection_background: Hsla::default(),
            placeholder: SharedString::default(),
            disabled: false,
            text_color: gpui::hsla(0.0, 0.0, 0.0, 0.0),
            placeholder_color: gpui::hsla(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn set_text_colors(&mut self, text: Hsla, placeholder_color: Hsla) {
        self.text_color = text;
        self.placeholder_color = placeholder_color;
    }

    pub fn set_selection_background(&mut self, color: Hsla) {
        self.selection_background = color;
    }

    pub fn value(&self) -> &SharedString {
        &self.value
    }

    pub fn set_value(&mut self, value: impl Into<SharedString>) {
        self.value = value.into();
        self.cursor_pos = self.value.len();
        self.selection = None;
    }

    pub fn set_placeholder(&mut self, text: impl Into<SharedString>) {
        self.placeholder = text.into();
    }

    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus_handle
    }

    pub fn insert_text(&mut self, text: &str, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if let Some(range) = self.selection.take() {
            let mut s: String = self.value.to_string();
            s.replace_range(range.clone(), "");
            self.cursor_pos = range.start;
            self.value = s.into();
        }
        let mut s: String = self.value.to_string();
        let pos = self.cursor_pos.min(s.len());
        for c in text.chars() {
            if c == '\x08' || c == '\x7f' {
                if pos > 0 {
                    s.remove(pos - 1);
                    self.cursor_pos = self.cursor_pos.saturating_sub(1);
                }
            } else {
                s.insert(pos, c);
                self.cursor_pos += 1;
            }
        }
        self.value = s.into();
        cx.emit(InputEvent::Change);
        cx.notify();
    }

    pub fn backspace(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if let Some(range) = self.selection.take() {
            let mut s: String = self.value.to_string();
            s.replace_range(range.clone(), "");
            self.cursor_pos = range.start;
            self.value = s.into();
            cx.emit(InputEvent::Change);
            cx.notify();
            return;
        }
        if self.cursor_pos == 0 {
            return;
        }
        let mut s: String = self.value.to_string();
        s.remove(self.cursor_pos - 1);
        self.cursor_pos -= 1;
        self.value = s.into();
        cx.emit(InputEvent::Change);
        cx.notify();
    }

    pub fn delete(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        if let Some(range) = self.selection.take() {
            let mut s: String = self.value.to_string();
            s.replace_range(range.clone(), "");
            self.cursor_pos = range.start;
            self.value = s.into();
            cx.emit(InputEvent::Change);
            cx.notify();
            return;
        }
        if self.cursor_pos >= self.value.len() {
            return;
        }
        let mut s: String = self.value.to_string();
        s.remove(self.cursor_pos);
        self.value = s.into();
        cx.emit(InputEvent::Change);
        cx.notify();
    }

    pub fn move_left(&mut self) {
        self.selection = None;
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        self.selection = None;
        self.cursor_pos = self.cursor_pos.min(self.value.len());
    }

    pub fn move_home(&mut self) {
        self.selection = None;
        self.cursor_pos = 0;
    }

    pub fn move_end(&mut self) {
        self.selection = None;
        self.cursor_pos = self.value.len();
    }

    pub fn select_all(&mut self) {
        self.selection = Some(0..self.value.len());
    }

    pub fn selected_value(&self) -> SharedString {
        self.selection
            .as_ref()
            .map(|range| self.value[range.clone()].into())
            .unwrap_or_default()
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }
}

impl Focusable for InputState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InputState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_placeholder = self.value.is_empty() && !self.placeholder.is_empty();
        let display_text = if is_placeholder {
            self.placeholder.clone()
        } else {
            self.value.clone()
        };
        let color = if is_placeholder {
            self.placeholder_color
        } else {
            self.text_color
        };

        if let Some(range) = &self.selection
            && !is_placeholder
            && !range.is_empty()
            && range.start < display_text.len()
            && range.end <= display_text.len()
        {
            let text = display_text.to_string();
            let before = text[..range.start].to_string();
            let selected = text[range.start..range.end].to_string();
            let after = text[range.end..].to_string();

            return div()
                .flex()
                .flex_1()
                .text_color(color)
                .child(before)
                .child(div().bg(self.selection_background).child(selected))
                .child(after);
        }

        div().flex().flex_1().text_color(color).child(display_text)
    }
}
