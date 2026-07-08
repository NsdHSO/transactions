use gpui::{App, Global, Window};

use crate::appearance::Appearance;
use crate::colors::ThemeColors;

pub struct ZalmoxisTheme {
    pub appearance: Appearance,
    pub colors: ThemeColors,
}

impl Global for ZalmoxisTheme {}

pub fn init(cx: &mut App) {
    cx.set_global(ZalmoxisTheme {
        appearance: Appearance::Light,
        colors: ThemeColors::light(),
    });
}

pub fn set_appearance(appearance: Appearance, cx: &mut App) {
    let colors = match appearance {
        Appearance::Light => ThemeColors::light(),
        Appearance::Dark => ThemeColors::dark(),
    };
    cx.set_global(ZalmoxisTheme { appearance, colors });
}

pub fn init_and_observe(window: &mut Window, cx: &mut App) {
    let initial: Appearance = window.appearance().into();
    set_appearance(initial, cx);
    let _ = window.observe_window_appearance(|window, cx| {
        set_appearance(window.appearance().into(), cx);
    });
}

pub trait ActiveTheme {
    fn zalmoxis_colors(&self) -> &ThemeColors;
    fn zalmoxis_appearance(&self) -> Appearance;
}

impl ActiveTheme for App {
    fn zalmoxis_colors(&self) -> &ThemeColors {
        &self.global::<ZalmoxisTheme>().colors
    }

    fn zalmoxis_appearance(&self) -> Appearance {
        self.global::<ZalmoxisTheme>().appearance
    }
}
