mod interface;
mod sidebar;
mod viewport;

pub use interface::Interface;

pub const COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
pub const COLOR_DARK_GRAY: (u8, u8, u8) = (37, 37, 37);
pub const COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerInteraction {
    CursorMove { x: i32, y: i32 },
    WindowLeftClick { x: i32, y: i32 },
    WindowRightClick { x: i32, y: i32 },
    KeyPress { key: KeyboardKey },
}

#[derive(PartialEq, Clone, Copy)]
pub enum KeyboardKey {
    Space,
}
