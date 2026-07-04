pub mod ghost;

pub use ghost::DividerGhost;

#[derive(Clone, Copy)]
pub struct DividerDrag;

pub const DIVIDER_WIDTH: f32 = 4.0;
pub const MIN_PANE: f32 = 100.0;
