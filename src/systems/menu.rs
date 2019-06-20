use crate::state::{Direction, GameState, Object, PlayerMode, SidebarMenu};

pub fn apply_open_menu(game: &mut GameState, menu: SidebarMenu) {
    game.open_menu = Some(menu);
    match menu {
        SidebarMenu::Navigation => {
            game.player_mode = PlayerMode::Focus;
        }
        SidebarMenu::Building => {
            game.player_mode = PlayerMode::PlaceObject {
                obj: Object::Forest,
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
