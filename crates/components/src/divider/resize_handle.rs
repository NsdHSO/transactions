use gpui::{
    AppContext, Context, DragMoveEvent, InteractiveElement, IntoElement, Render,
    StatefulInteractiveElement, Styled, Window, div, px, rgb,
};
use zalmoxis::ActiveTheme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
struct ResizeHandleDrag;

pub struct ResizeHandle {
    pub value: f32,
    orientation: Orientation,
    pub min: f32,
    pub max: f32,
}

impl ResizeHandle {
    pub fn vertical(initial: f32, min: f32, max: f32) -> Self {
        Self::new(Orientation::Vertical, initial, min, max)
    }

    pub fn horizontal(initial: f32, min: f32, max: f32) -> Self {
        Self::new(Orientation::Horizontal, initial, min, max)
    }

    fn new(orientation: Orientation, initial: f32, min: f32, max: f32) -> Self {
        Self {
            value: initial,
            orientation,
            min,
            max,
        }
    }
}

impl Render for ResizeHandle {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let orientation = self.orientation;
        let colors = cx.zalmoxis_colors();

        let base = div().id("resize-handle").bg(colors.outline_variant);

        let base = match orientation {
            Orientation::Vertical => base.h(px(2.0)).w_full().cursor_ns_resize(),
            Orientation::Horizontal => base.w(px(2.0)).h_full().cursor_ew_resize(),
        };

        base.hover(|s| s.bg(rgb(0x00aaff)))
            .on_drag(ResizeHandleDrag, move |_, pos, _, cx| {
                cx.new(|_| ResizeHandleGhost {
                    pos: match orientation {
                        Orientation::Horizontal => pos.x.into(),
                        Orientation::Vertical => pos.y.into(),
                    },
                    orientation,
                })
            })
            .on_drag_move(
                cx.listener(|s, e: &DragMoveEvent<ResizeHandleDrag>, window, cx| {
                    let total = f32::from(match s.orientation {
                        Orientation::Horizontal => window.viewport_size().width,
                        Orientation::Vertical => window.viewport_size().height,
                    });
                    let mouse = f32::from(match s.orientation {
                        Orientation::Horizontal => e.event.position.x,
                        Orientation::Vertical => e.event.position.y,
                    });
                    s.value = match s.orientation {
                        Orientation::Horizontal => mouse.clamp(s.min, s.max),
                        Orientation::Vertical => (total - mouse).clamp(s.min, s.max),
                    };
                    cx.notify();
                }),
            )
    }
}

struct ResizeHandleGhost {
    pos: f32,
    orientation: Orientation,
}

impl Render for ResizeHandleGhost {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.orientation {
            Orientation::Horizontal => div()
                .absolute()
                .w(px(1.0))
                .h_full()
                .bg(rgb(0x00aaff))
                .left(px(self.pos))
                .top(px(0.0))
                .into_element(),
            Orientation::Vertical => div()
                .absolute()
                .h(px(1.0))
                .w_full()
                .bg(rgb(0x00aaff))
                .top(px(self.pos))
                .left(px(0.0))
                .into_element(),
        }
    }
}
