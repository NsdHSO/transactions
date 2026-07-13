mod app_root;

use app_root::AppRoot;
use gpui::{AppContext, AsyncApp, Bounds, WindowBounds, WindowOptions};

fn main() {
    let app = gpui_platform::application();

    app.run(move |cx: &mut gpui::App| {
        zalmoxis::init(cx);

        cx.spawn(async move |cx: &mut AsyncApp| {
            cx.open_window(
                WindowOptions {
                    focus: true,
                    window_bounds: Some(WindowBounds::Maximized(Bounds::default())),
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
