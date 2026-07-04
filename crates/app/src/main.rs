// use std::ops::Rem;

use components::panes::{HelloWorld, HelloWorld1};
use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, rems, size};
use split::Split;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(rems(1000.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let left = cx.new(|_| HelloWorld {
                    text: "World".into(),
                });
                let right = cx.new(|_| HelloWorld1 {
                    text: "World1".into(),
                });
                cx.new(|_| Split {
                    left,
                    right,
                    left_width: 500.0,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
