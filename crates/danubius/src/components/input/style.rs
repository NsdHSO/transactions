#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    Outlined,
    Filled,
    Standard,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

impl InputSize {
    pub fn height_px(&self) -> f32 {
        match self {
            InputSize::Small => 32.0,
            InputSize::Medium => 40.0,
            InputSize::Large => 48.0,
        }
    }

    pub fn font_size_px(&self) -> f32 {
        match self {
            InputSize::Small => 12.0,
            InputSize::Medium => 14.0,
            InputSize::Large => 16.0,
        }
    }

    pub fn padding_x_px(&self) -> f32 {
        match self {
            InputSize::Small => 8.0,
            InputSize::Medium => 12.0,
            InputSize::Large => 16.0,
        }
    }
}
