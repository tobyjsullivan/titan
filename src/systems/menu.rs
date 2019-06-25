use crate::state::board::{Direction, Structure};
use crate::state::game::{GameState, PlayerMode};
use crate::state::menu::SidebarMenu;

pub fn apply_open_menu(game: &mut GameState, menu: SidebarMenu) {
    game.open_menu = Some(menu);
    match menu {
        SidebarMenu::Navigation => {
            game.player_mode = PlayerMode::Focus;
        }
        SidebarMenu::Building => {
            game.player_mode = PlayerMode::PlaceStructure {
                structure: Structure::TrainStation,
                orientation: Direction::North,
            };
        }
        SidebarMenu::Demolish => {
            game.player_mode = PlayerMode::RaiseLower { radius: 0 };
        }
        // TODO (toby)
        _ => {}
    }
}
