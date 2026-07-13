use components::HelloWorld;
use components::panes::{HelloWorld1, Toolbar};
use danubius::components::dropdown::{Dropdown, MenuEntry};
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use split::Split;
use zalmoxis::{ActiveTheme, Appearance, set_appearance};

pub struct AppRoot {
    toolbar: Entity<Toolbar>,
    split: Entity<Split>,
}

impl AppRoot {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let toolbar = cx.new(|_| Toolbar::new());
        let left = cx.new(|_| HelloWorld {
            text: "World".into(),
        });
        let right = cx.new(|cx| HelloWorld1::new(window, cx));
        let split = cx.new(|_| Split {
            left,
            right,
            left_width: 500.0,
        });

        let _ = cx.observe(
            &toolbar,
            |_: &mut Self, _: Entity<Toolbar>, cx: &mut Context<Self>| {
                cx.notify();
            },
        );

        Self { toolbar, split }
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toolbar_entity = self.toolbar.clone();
        let open = self.toolbar.read(cx).dropdown_open;

        let theme_label = match cx.zalmoxis_appearance() {
            Appearance::Light => "☾  Dark theme",
            Appearance::Dark => "☀  Light theme",
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.toolbar.clone())
            .child(div().flex_1().child(self.split.clone()))
            .when(open, |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(48.0))
                        .right(px(16.0))
                        .child(Dropdown::new(vec![MenuEntry::new(
                            theme_label,
                            move |window, cx| {
                                toolbar_entity.update(cx, |this, cx| {
                                    this.dropdown_open = false;
                                    cx.notify();
                                });
                                let current = cx.zalmoxis_appearance();
                                let new = match current {
                                    Appearance::Light => Appearance::Dark,
                                    Appearance::Dark => Appearance::Light,
                                };
                                set_appearance(new, cx);
                                window.refresh();
                            },
                        )]))
                        .into_element(),
                )
            })
    }
}
