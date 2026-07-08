use components::{HelloWorld, panes::HelloWorld1};
use gpui::{AsyncApp, Bounds, WindowBounds, WindowOptions, *};
use split::Split;

struct Root {
    split: Entity<Split>,
}

impl Root {
    fn new(split: Entity<Split>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { split }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.split.clone()
    }
}

fn main() {
    let app = gpui_platform::application();

    app.run(move |cx: &mut App| {
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
