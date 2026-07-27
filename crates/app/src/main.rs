mod app_root;

use app_root::AppRoot;
use gpui::{AppContext, AsyncApp, WindowBounds, WindowOptions};

fn main() {
    let app = gpui_platform::application();

    app.run(move |cx: &mut gpui::App| {
        zalmoxis::init(cx);

        let display_id = cx
            .displays()
            .iter()
            .min_by_key(|d| {
                let w: f32 = d.bounds().size.width.into();
                (w * 100.0) as i64
            })
            .map(|d| d.id());
        let bounds = display_id
            .and_then(|id| cx.find_display(id))
            .map(|d| d.bounds());

        cx.spawn(async move |cx: &mut AsyncApp| {
            cx.open_window(
                WindowOptions {
                    focus: true,
                    display_id,
                    window_bounds: Some(WindowBounds::Maximized(bounds.unwrap_or_default())),
                    ..Default::default()
                },
                |window, cx| {
                    zalmoxis::init_and_observe(window, cx);
                    cx.new(|cx| AppRoot::new(window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
