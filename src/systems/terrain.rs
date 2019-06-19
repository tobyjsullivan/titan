use crate::action::GameAction;
use crate::state::{GameBoard, GameState, Vertex};

const MIN_HEIGHT: u8 = 0;
const MAX_HEIGHT: u8 = 6;

pub fn apply(game: &mut GameState, action: GameAction) {
    match action {
        GameAction::LowerTerrain => lower_terrain(game),
        GameAction::RaiseTerrain => raise_terrain(game),
    }
}

fn lower_terrain(game: &mut GameState) {
    if let Some(block) = game.highlighted_block {
        let v = Vertex {
            x: block.x,
            y: block.y,
        };
        match lower_vertex(&mut game.board, v) {
            Ok(()) => println!("Vertex lowered."),
            Err(()) => println!("Lowering failed."),
        }
    }
}

fn lower_vertex(mut board: &mut GameBoard, vertex: Vertex) -> Result<(), ()> {
    println!("Trying to lower: {:?}", vertex);
    let prior_height = board.vertex_height(vertex);
    if prior_height <= MIN_HEIGHT {
        println!("Already min.");
        return Err(());
    }

    // Vertices on the edge of the board cannot be raised or lowered.
    if vertex.is_edge_vertex() {
        println!("Is edge vertex.");
        return Err(());
    }

    // Every surrounding vertex must be at most the current height of this vertex in order to lower.
    // If a surrounding vertex is not the necessary height, lower that vertex first.
    // This is a recursive operation.
    for y in (vertex.y - 1)..(vertex.y + 2) {
        for x in (vertex.x - 1)..(vertex.x + 2) {
            let v = Vertex { x, y };
            let h = board.vertex_height(v);
            // This will skip the current vertex.
            if h > prior_height {
                // TODO (toby): Avoid making any changes if the prerequisites or unsatisfied
                // rather than bailing out part-way through.
                lower_vertex(&mut board, v)?;
            }
        }
    }

    board.set_vertex_height(vertex, prior_height - 1);
    Ok(())
}

fn raise_terrain(game: &mut GameState) {
    if let Some(block) = game.highlighted_block {
        let v = Vertex {
            x: block.x,
            y: block.y,
        };
        match raise_vertex(&mut game.board, v) {
            Ok(()) => println!("Vertex raised."),
            Err(()) => println!("Raising failed."),
        }
    }
}

fn raise_vertex(mut board: &mut GameBoard, vertex: Vertex) -> Result<(), ()> {
    println!("Trying to raise: {:?}", vertex);
    let prior_height = board.vertex_height(vertex);
    if prior_height >= MAX_HEIGHT {
        println!("Already max.");
        return Err(());
    }

    // Vertices on the edge of the board cannot be raised or lowered.
    if vertex.is_edge_vertex() {
        println!("Is edge vertex.");
        return Err(());
    }

    // Every surrounding vertex must be at least the current height of this vertex in order to raise.
    // If a surrounding vertex is not the necessary height, raise that vertex first.
    // This is a recursive operation.
    for y in (vertex.y - 1)..(vertex.y + 2) {
        for x in (vertex.x - 1)..(vertex.x + 2) {
            let v = Vertex { x, y };
            let h = board.vertex_height(v);
            // This will skip the current vertex.
            if h < prior_height {
                // TODO (toby): Avoid making any changes if the prerequisites or unsatisfied
                // rather than bailing out part-way through.
                raise_vertex(&mut board, v)?;
            }
        }
    }

    board.set_vertex_height(vertex, prior_height + 1);
    Ok(())
}
