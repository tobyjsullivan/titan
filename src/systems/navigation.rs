use crate::state::GameState;

pub fn apply_focus(game: &mut GameState) {
    if let Some(block) = game.highlighted_block {
        game.focal_point = block.into();
    }
}
