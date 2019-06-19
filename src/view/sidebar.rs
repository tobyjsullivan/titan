use crate::action::GameAction;
use crate::controller::PlayerAction;
use crate::state::GameState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const COLOR_SIDEBAR: (u8, u8, u8) = (150, 150, 150);

pub struct Sidebar {
    width: u32,
    height: u32,
}

impl Sidebar {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn map_player_action(
        &self,
        game: &GameState,
        player_action: PlayerAction,
    ) -> Option<GameAction> {
        match &player_action {
            // TODO (toby)
            _ => None,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
        canvas.set_draw_color(Color::from(COLOR_SIDEBAR));
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height))?;

        // TODO (toby)

        Ok(())
    }
}
