use gpui::Hsla;

const P: fn(f32, f32, f32, f32) -> Hsla = gpui::hsla;

macro_rules! hsl {
    ($h:expr, $s:expr, $l:expr) => {
        P($h, $s, $l, 1.0)
    };
}

#[derive(Clone, Debug)]
pub struct ThemeColors {
    // Primary
    pub primary: Hsla,
    pub on_primary: Hsla,
    pub primary_container: Hsla,
    pub on_primary_container: Hsla,
    // Secondary
    pub secondary: Hsla,
    pub on_secondary: Hsla,
    pub secondary_container: Hsla,
    pub on_secondary_container: Hsla,
    // Tertiary
    pub tertiary: Hsla,
    pub on_tertiary: Hsla,
    pub tertiary_container: Hsla,
    pub on_tertiary_container: Hsla,
    // Surface
    pub surface: Hsla,
    pub on_surface: Hsla,
    pub surface_variant: Hsla,
    pub on_surface_variant: Hsla,
    // Background
    pub background: Hsla,
    pub on_background: Hsla,
    // Error
    pub error: Hsla,
    pub on_error: Hsla,
    pub error_container: Hsla,
    pub on_error_container: Hsla,
    // Outline
    pub outline: Hsla,
    pub outline_variant: Hsla,
    // Inverse
    pub inverse_surface: Hsla,
    pub inverse_on_surface: Hsla,
    pub inverse_primary: Hsla,
    // Misc
    pub scrim: Hsla,
    pub shadow: Hsla,
}

impl ThemeColors {
    pub fn light() -> Self {
        Self {
            primary: hsl!(263.0, 0.80, 0.50),
            on_primary: hsl!(0.0, 0.0, 1.0),
            primary_container: hsl!(265.0, 0.70, 0.92),
            on_primary_container: hsl!(265.0, 0.80, 0.18),
            secondary: hsl!(264.0, 0.18, 0.45),
            on_secondary: hsl!(0.0, 0.0, 1.0),
            secondary_container: hsl!(265.0, 0.30, 0.88),
            on_secondary_container: hsl!(265.0, 0.25, 0.16),
            tertiary: hsl!(290.0, 0.40, 0.40),
            on_tertiary: hsl!(0.0, 0.0, 1.0),
            tertiary_container: hsl!(290.0, 0.50, 0.88),
            on_tertiary_container: hsl!(290.0, 0.50, 0.14),
            surface: hsl!(270.0, 0.06, 0.98),
            on_surface: hsl!(270.0, 0.08, 0.12),
            surface_variant: hsl!(270.0, 0.06, 0.92),
            on_surface_variant: hsl!(270.0, 0.04, 0.38),
            background: hsl!(270.0, 0.06, 0.98),
            on_background: hsl!(270.0, 0.08, 0.12),
            error: hsl!(0.0, 0.80, 0.50),
            on_error: hsl!(0.0, 0.0, 1.0),
            error_container: hsl!(0.0, 0.80, 0.92),
            on_error_container: hsl!(0.0, 0.80, 0.18),
            outline: hsl!(270.0, 0.04, 0.55),
            outline_variant: hsl!(270.0, 0.06, 0.85),
            inverse_surface: hsl!(270.0, 0.08, 0.18),
            inverse_on_surface: hsl!(270.0, 0.06, 0.96),
            inverse_primary: hsl!(265.0, 0.70, 0.85),
            scrim: hsl!(0.0, 0.0, 0.0),
            shadow: hsl!(0.0, 0.0, 0.0),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: hsl!(265.0, 0.70, 0.82),
            on_primary: hsl!(265.0, 0.80, 0.18),
            primary_container: hsl!(265.0, 0.70, 0.28),
            on_primary_container: hsl!(265.0, 0.70, 0.92),
            secondary: hsl!(264.0, 0.18, 0.72),
            on_secondary: hsl!(265.0, 0.20, 0.16),
            secondary_container: hsl!(265.0, 0.18, 0.28),
            on_secondary_container: hsl!(265.0, 0.20, 0.88),
            tertiary: hsl!(290.0, 0.50, 0.76),
            on_tertiary: hsl!(290.0, 0.50, 0.14),
            tertiary_container: hsl!(290.0, 0.40, 0.26),
            on_tertiary_container: hsl!(290.0, 0.50, 0.88),
            surface: hsl!(270.0, 0.08, 0.10),
            on_surface: hsl!(270.0, 0.06, 0.94),
            surface_variant: hsl!(270.0, 0.08, 0.18),
            on_surface_variant: hsl!(270.0, 0.04, 0.70),
            background: hsl!(270.0, 0.08, 0.08),
            on_background: hsl!(270.0, 0.06, 0.94),
            error: hsl!(0.0, 0.70, 0.70),
            on_error: hsl!(0.0, 0.70, 0.18),
            error_container: hsl!(0.0, 0.70, 0.30),
            on_error_container: hsl!(0.0, 0.70, 0.90),
            outline: hsl!(270.0, 0.04, 0.55),
            outline_variant: hsl!(270.0, 0.06, 0.30),
            inverse_surface: hsl!(270.0, 0.06, 0.94),
            inverse_on_surface: hsl!(270.0, 0.08, 0.12),
            inverse_primary: hsl!(263.0, 0.80, 0.50),
            scrim: hsl!(0.0, 0.0, 0.0),
            shadow: hsl!(0.0, 0.0, 0.0),
        }
    }
}
