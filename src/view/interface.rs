use super::screens::building::BuyBuildingScreen;
use super::sidebar::Sidebar;
use super::text::DynamicText;
use super::viewport::Viewport;
use super::{
    KeyboardKey, PlayerInteraction, ScreenState, COLOR_DARK_GRAY, DIALOG_HEIGHT, DIALOG_WIDTH,
    TEXT_HEIGHT,
};
use crate::action::GameAction;
use crate::state::game::GameState;
use crate::state::menu::building::BuyBuildingScreenState;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use std::rc::Rc;

pub struct Interface {
    buy_building_screen: BuyBuildingScreen,
    viewport: Viewport,
    screen: ScreenState,
    sidebar: Sidebar,
}

impl Interface {
    pub fn new<T>(texture_creator: TextureCreator<T>, screen: ScreenState) -> Self {
        let dynamic_text = DynamicText::new(&texture_creator, screen);

        Self {
            buy_building_screen: BuyBuildingScreen::new(
                &texture_creator,
                Rc::new(dynamic_text),
                screen,
            ),
            viewport: Viewport::new(screen),
            screen,
            sidebar: Sidebar::new(&texture_creator, screen),
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
        if let BuyBuildingScreenState::Visible {
            selected_building,
            selected_category,
        } = game.buy_building_screen
        {
            self.buy_building_screen.render(canvas)?;
        }

        canvas.present();

        Ok(())
    }

    fn window_panel(&self, x: i32, y: i32) -> WindowPanel {
        let (sidebar_width, _) = self.sidebar.size();
        if x <= sidebar_width as i32 {
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
