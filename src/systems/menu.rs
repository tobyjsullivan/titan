use crate::state::board::{Direction, Structure};
use crate::state::game::{GameState, PlayerMode};
use crate::state::menu::building::{Building, BuyBuildingScreenState, Category};
use crate::state::menu::sidebar::SidebarMenu;

pub fn apply_open_menu(game: &mut GameState, menu: SidebarMenu) {
    game.open_menu = Some(menu);
    match menu {
        SidebarMenu::Navigation => {
            game.player_mode = PlayerMode::Focus;
        }
        SidebarMenu::Building => {
            apply_display_building_screen(game);
            // game.player_mode = PlayerMode::PlaceStructure {
            //     structure: Structure::TrainStation,
            //     orientation: Direction::North,
            // };
        }
        SidebarMenu::Demolish => {
            game.player_mode = PlayerMode::RaiseLower { radius: 0 };
        }
        // TODO (toby)
        _ => {}
    }
}

pub fn apply_display_building_screen(game: &mut GameState) {
    game.buy_building_screen = BuyBuildingScreenState::Visible {
        selected_building: Building::TruckDepot,
        selected_category: Category::Terminals,
    };
}
