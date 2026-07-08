#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    pub fn height_px(&self) -> f32 {
        match self {
            ButtonSize::Small => 28.0,
            ButtonSize::Medium => 36.0,
            ButtonSize::Large => 44.0,
        }
    }

    pub fn font_size_px(&self) -> f32 {
        match self {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 14.0,
            ButtonSize::Large => 16.0,
        }
    }

    pub fn padding_x_px(&self) -> f32 {
        match self {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 20.0,
            ButtonSize::Large => 28.0,
        }
    }
}
