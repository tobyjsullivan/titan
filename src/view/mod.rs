mod interface;
mod screens;
mod sidebar;
pub mod text;
mod viewport;

pub use interface::Interface;

pub(self) const COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
pub(self) const COLOR_DARK_GRAY: (u8, u8, u8) = (37, 37, 37);
pub(self) const COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);

pub(self) const TEXT_HEIGHT: u32 = 13;

pub(self) const SIDEBAR_WIDTH: u32 = 160;
pub(self) const DIALOG_WIDTH: u32 = 640;
pub(self) const DIALOG_HEIGHT: u32 = 480;

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

#[derive(PartialEq, Clone, Copy)]
pub struct ScreenState {
    width: u32,
    height: u32,
    hidpi_scale: HiDpiScale,
}

impl ScreenState {
    pub fn new(
        window_width: u32,
        window_height: u32,
        drawable_width: u32,
        drawable_height: u32,
    ) -> Self {
        let scale_x = drawable_width / window_width;
        let scale_y = drawable_height / window_height;

        let scaling = match (scale_x, scale_y) {
            (1, 1) => HiDpiScale::None,
            (2, 2) => HiDpiScale::X2,
            (3, 3) => HiDpiScale::X3,
            (4, 4) => HiDpiScale::X4,
            (_, _) => HiDpiScale::None, // No support for fractional or asymetric scaling.
        };

        Self {
            width: window_width,
            height: window_height,
            hidpi_scale: scaling,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (
            self.hidpi_scale.scale(self.width as i32) as u32,
            self.hidpi_scale.scale(self.height as i32) as u32,
        )
    }

    pub fn scale_x(&self, x: i32) -> i32 {
        self.hidpi_scale.scale(x)
    }

    pub fn scale_y(&self, y: i32) -> i32 {
        self.hidpi_scale.scale(y)
    }
}

#[derive(PartialEq, Clone, Copy)]
enum HiDpiScale {
    None,
    X2,
    X3,
    X4,
}

impl HiDpiScale {
    fn scale(&self, input: i32) -> i32 {
        match self {
            HiDpiScale::None => input,
            HiDpiScale::X2 => input * 2,
            HiDpiScale::X3 => input * 3,
            HiDpiScale::X4 => input * 4,
        }
    }
}
