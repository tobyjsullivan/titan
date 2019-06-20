use crate::action::GameAction;
use crate::state::GameState;
use crate::view::sidebar::Sidebar;
use crate::view::viewport::Viewport;

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

pub fn map_player_action(
    sidebar: &Sidebar,
    viewport: &Viewport,
    game: &GameState,
    player_action: PlayerAction,
) -> Option<GameAction> {
    match player_action {
        cursor_move @ PlayerAction::CursorMove {
            panel: WindowPanel::Sidebar,
            ..
        } => sidebar.map_player_action(game, cursor_move),
        click @ PlayerAction::WindowLeftClick {
            panel: WindowPanel::Sidebar,
            ..
        } => sidebar.map_player_action(game, click),
        click @ PlayerAction::WindowRightClick {
            panel: WindowPanel::Sidebar,
            ..
        } => sidebar.map_player_action(game, click),
        cursor_move @ PlayerAction::CursorMove {
            panel: WindowPanel::Viewport,
            ..
        } => viewport.map_player_action(game, cursor_move),
        click @ PlayerAction::WindowLeftClick {
            panel: WindowPanel::Viewport,
            ..
        } => viewport.map_player_action(game, click),
        click @ PlayerAction::WindowRightClick {
            panel: WindowPanel::Viewport,
            ..
        } => viewport.map_player_action(game, click),
        PlayerAction::PressSpace => viewport.map_player_action(game, PlayerAction::PressSpace),
    }
}
