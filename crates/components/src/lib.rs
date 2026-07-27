pub mod divider;
pub mod panes;
pub mod route;

pub use divider::{
    BottomBarDrag, BottomBarGhost, DIVIDER_WIDTH, DividerDrag, DividerGhost, MIN_PANE, Orientation,
    ResizeHandle,
};
pub use panes::{
    bottom_bar::BottomBar, hello_world::HelloWorld, hello_world1::HelloWorld1,
    info_bottom_bar::InfoBottomBar,
};
