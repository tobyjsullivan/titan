use crate::state::{Block, GameState, SidebarMenu};

pub fn apply_focus(game: &mut GameState) {
    if let Some(block) = game.highlighted_block {
        game.focal_point = block.into();
    }
}

pub fn apply_hover(game: &mut GameState, block: Option<Block>) {
    if let Some(block) = block {
        game.highlighted_block = Some(block);
        game.highlighted_button = None;
    } else {
        game.highlighted_block = None;
    }
}

pub fn apply_sidebar_hover(game: &mut GameState, button: Option<SidebarMenu>) {
    if let Some(button) = button {
        game.highlighted_block = None;
        game.highlighted_button = Some(button);
    } else {
        game.highlighted_button = None;
    }
}
