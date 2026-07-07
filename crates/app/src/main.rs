use components::panes::{HelloWorld, HelloWorld1};
use gpui::{AsyncApp, Bounds, WindowBounds, WindowOptions, *};
use gpui_component::Root;
use split::Split;

fn main() {
    let app = gpui_platform::application();

    app.run(move |cx: &mut App| {
        gpui_component::init(cx);

        cx.spawn(async move |cx: &mut AsyncApp| {
            cx.open_window(
                WindowOptions {
                    focus: true,
                    window_bounds: Some(WindowBounds::Maximized(Bounds::default())),
                    ..Default::default()
                },
                |window, cx| {
                    let left = cx.new(|_| HelloWorld {
                        text: "World".into(),
                    });
                    let right = cx.new(|cx| HelloWorld1::new(window, cx));
                    let split = cx.new(|_| Split {
                        left,
                        right,
                        left_width: 500.0,
                    });
                    cx.new(|cx| Root::new(split, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
