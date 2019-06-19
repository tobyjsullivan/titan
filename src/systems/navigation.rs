use crate::state::{Block, GameState};

pub fn apply_focus(game: &mut GameState) {
    if let Some(block) = game.highlighted_block {
        game.focal_point = block.into();
    }
}

pub fn apply_hover(game: &mut GameState, block: Option<Block>) {
    game.highlighted_block = block;
}
