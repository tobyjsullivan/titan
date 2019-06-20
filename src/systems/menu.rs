use crate::state::{GameState, PlayerMode};

pub fn apply_set_player_mode(game: &mut GameState, mode: PlayerMode) {
    game.player_mode = mode;
}
