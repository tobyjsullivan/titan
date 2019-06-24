use crate::state::{Direction, GameState, PlayerMode};

pub fn apply_rotate_structure(game: &mut GameState) {
    if let PlayerMode::PlaceStructure {
        structure,
        orientation,
    } = game.player_mode
    {
        let next_orientation = match orientation {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        game.player_mode = PlayerMode::PlaceStructure {
            structure,
            orientation: next_orientation,
        };
    }
}

pub fn apply_place_structure(game: &mut GameState) {
    if let (
        PlayerMode::PlaceStructure {
            structure,
            orientation,
        },
        Some(block),
    ) = (game.player_mode, game.highlighted_block)
    {
        game.board.place_structure(structure, orientation, block);
    }
}
