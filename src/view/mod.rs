pub mod sidebar;
pub mod viewport;

use crate::action::GameAction;
use crate::state::GameState;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sidebar::Sidebar;
use viewport::Viewport;

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

pub struct Interface {
    viewport: Viewport,
    sidebar: Sidebar,
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

    pub fn map_player_interaction(
        &self,
        game: &GameState,
        player_action: PlayerInteraction,
    ) -> Option<GameAction> {
        match player_action {
            PlayerInteraction::CursorMove { x, y } => match self.window_panel(x, y) {
                WindowPanel::Sidebar => self.sidebar.cursor_move_action(x, y),
                WindowPanel::Viewport => self.viewport.cursor_move_action(game, x, y),
            },
            PlayerInteraction::WindowLeftClick { x, y } => match self.window_panel(x, y) {
                WindowPanel::Sidebar => self.sidebar.left_click_action(x, y),
                WindowPanel::Viewport => self.viewport.left_click_action(game),
            },
            PlayerInteraction::WindowRightClick { x, y } => match self.window_panel(x, y) {
                WindowPanel::Sidebar => None,
                WindowPanel::Viewport => self.viewport.right_click_action(game),
            },
            PlayerInteraction::KeyPress {
                key: KeyboardKey::Space,
            } => self.viewport.spacebar_action(game),
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

#[derive(PartialEq, Clone, Copy)]
enum WindowPanel {
    Sidebar,
    Viewport,
}
