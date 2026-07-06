use components::panes::{HelloWorld, HelloWorld1};
use gpui::{App, AppContext, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions};
use split::Split;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                focus: true,
                titlebar: Some(TitlebarOptions {
                    title: Some("My App".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                window_bounds: Some(WindowBounds::Maximized(Bounds::default())),
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
