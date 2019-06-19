use crate::action::GameAction;
use crate::state::{GameState, Vertex};

const MIN_HEIGHT: u8 = 0;
const MAX_HEIGHT: u8 = 6;

pub fn apply(game: &mut GameState, action: GameAction) {
    match action {
        GameAction::LowerTerrain => lower_terrain(game),
        GameAction::RaiseTerrain => raise_terrain(game),
    }
}

fn lower_terrain(game: &mut GameState) {
    // TODO (toby): lower any dependent vertices before this one.
    if let Some(block) = game.highlighted_block {
        let v = Vertex {
            x: block.x,
            y: block.y,
        };
        let prior_height = game.board.vertex_height(v);
        if prior_height > MIN_HEIGHT {
            game.board.set_vertex_height(v, prior_height - 1);
        }
    }
}

fn raise_terrain(game: &mut GameState) {
    // TODO (toby): raise any dependent vertices before this one.
    if let Some(block) = game.highlighted_block {
        let v = Vertex {
            x: block.x,
            y: block.y,
        };
        let prior_height = game.board.vertex_height(v);
        if prior_height < MAX_HEIGHT {
            game.board.set_vertex_height(v, prior_height + 1);
        }
    }
}
