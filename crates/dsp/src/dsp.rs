use audio::components::audio::{AudioCapture, AudioProcessInfo};
use components::divider::ResizeHandle;
use components::panes::BottomBar;
use components::{InfoBottomBar, LeftBar};
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb,
};
use log::debug;
use std::time::Duration;
use zalmoxis::ActiveTheme;

pub struct Dsp {
    bottom_bar: Entity<BottomBar>,
    info_bar: Entity<InfoBottomBar>,
    left_bar: Entity<LeftBar>,
    handle: Entity<ResizeHandle>,
    _audio: AudioCapture,
    audio_process_info: Vec<AudioProcessInfo>,
}

impl Dsp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let handle = cx.new(|_| ResizeHandle::vertical(32.0, 32.0, 800.0));

        let audio = AudioCapture::new();
        let procs = audio.processes.clone();

        cx.spawn(async move |this, cx| {
            loop {
                let list = procs.lock().ok().map(|g| g.clone());
                let _ = this.update(cx, |this, cx| {
                    if let Some(ref processes) = list {
                        this.audio_process_info = processes.clone();
                        println!("Audio process info: {:?}", processes);
                    }
                    cx.notify();
                });
                cx.background_executor().timer(Duration::from_secs(2)).await;
            }
        })
        .detach();

        let _ = cx.observe(&handle, |_, _handle, cx| {
            cx.notify();
        });

        Self {
            bottom_bar: cx.new(|_| BottomBar),
            left_bar: cx.new(|_| LeftBar),
            info_bar: cx.new(|_| InfoBottomBar),
            handle,
            _audio: audio,
            audio_process_info: vec![],
        }
    }
}

impl Render for Dsp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.zalmoxis_colors();
        let height = self.handle.read(cx).value;

        div()
            .flex()
            .flex_row()
            .bg(colors.background)
            .size_full()
            .child(self.left_bar.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .justify_center()
                            .items_center()
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .child("DSP Screen"),
                    )
                    .child(self.handle.clone())
                    .child(div().h(px(height)).child(self.bottom_bar.clone()))
                    .child(div().h(px(30.0)).child(self.info_bar.clone())),
            )
    }
}
