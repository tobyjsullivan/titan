use crate::action::GameAction;
use crate::state::GameState;
use crate::view::viewport::Viewport;

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerAction {
    WindowLeftClick { x: i32, y: i32 },
    WindowRightClick { x: i32, y: i32 },
}

pub fn map_player_action(
    viewport: &Viewport,
    game: &GameState,
    player_action: PlayerAction,
) -> Option<GameAction> {
    // TODO (toby): Handle clicks and actions on components other than the viewport.
    viewport.map_player_action(game, player_action)
}
