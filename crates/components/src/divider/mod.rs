pub mod ghost;
pub mod resize_handle;

pub use ghost::{BottomBarGhost, DividerGhost};
pub use resize_handle::{Orientation, ResizeHandle};

#[derive(Clone, Copy)]
pub struct DividerDrag;

#[derive(Clone, Copy)]
pub struct BottomBarDrag;

pub const DIVIDER_WIDTH: f32 = 4.0;
pub const MIN_PANE: f32 = 100.0;
