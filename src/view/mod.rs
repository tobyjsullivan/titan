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
pub enum WindowPanel {
    Sidebar,
    Viewport,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerAction {
    CursorMove { panel: WindowPanel, x: i32, y: i32 },
    WindowLeftClick { panel: WindowPanel, x: i32, y: i32 },
    WindowRightClick { panel: WindowPanel, x: i32, y: i32 },
    PressSpace,
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

    pub fn map_player_action(
        &self,
        game: &GameState,
        player_action: PlayerAction,
    ) -> Option<GameAction> {
        match player_action {
            cursor_move @ PlayerAction::CursorMove {
                panel: WindowPanel::Sidebar,
                ..
            } => self.sidebar.map_player_action(game, cursor_move),
            click @ PlayerAction::WindowLeftClick {
                panel: WindowPanel::Sidebar,
                ..
            } => self.sidebar.map_player_action(game, click),
            click @ PlayerAction::WindowRightClick {
                panel: WindowPanel::Sidebar,
                ..
            } => self.sidebar.map_player_action(game, click),
            cursor_move @ PlayerAction::CursorMove {
                panel: WindowPanel::Viewport,
                ..
            } => self.viewport.map_player_action(game, cursor_move),
            click @ PlayerAction::WindowLeftClick {
                panel: WindowPanel::Viewport,
                ..
            } => self.viewport.map_player_action(game, click),
            click @ PlayerAction::WindowRightClick {
                panel: WindowPanel::Viewport,
                ..
            } => self.viewport.map_player_action(game, click),
            PlayerAction::PressSpace => self
                .viewport
                .map_player_action(game, PlayerAction::PressSpace),
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
