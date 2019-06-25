use super::sidebar::Sidebar;
use super::viewport::Viewport;
use super::{KeyboardKey, PlayerInteraction, COLOR_DARK_GRAY};
use crate::action::GameAction;
use crate::state::game::GameState;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;

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

    fn window_panel(&self, x: i32, y: i32) -> WindowPanel {
        if x <= self.sidebar_width as i32 {
            WindowPanel::Sidebar
        } else {
            WindowPanel::Viewport
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum WindowPanel {
    Sidebar,
    Viewport,
}
