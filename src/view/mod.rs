pub mod sidebar;
pub mod viewport;

use crate::state::GameState;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sidebar::Sidebar;
use viewport::Viewport;

pub const COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
pub const COLOR_DARK_GRAY: (u8, u8, u8) = (37, 37, 37);
pub const COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);

#[derive(PartialEq, Clone, Copy)]
pub enum WindowPanel {
    Sidebar,
    Viewport,
}

pub struct Interface {
    pub viewport: Viewport,
    pub sidebar: Sidebar,
    sidebar_width: u32,
}

impl Interface {
    pub fn new<T>(
        texture_creator: TextureCreator<T>,
        window_width: u32,
        window_height: u32,
        text_height: u32,
        sidebar_width: u32,
    ) -> Self {
        Self {
            viewport: Viewport::new(window_width - sidebar_width, window_height, sidebar_width),
            sidebar: Sidebar::new(texture_creator, sidebar_width, window_height, text_height),
            sidebar_width,
        }
    }

    pub fn window_panel(&self, x: i32, y: i32) -> WindowPanel {
        if x <= self.sidebar_width as i32 {
            WindowPanel::Sidebar
        } else {
            WindowPanel::Viewport
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
        canvas.set_draw_color(COLOR_DARK_GRAY);
        canvas.clear();

        self.viewport.render(canvas, &game)?;
        self.sidebar.render(canvas, &game)?;
        canvas.present();

        Ok(())
    }
}
