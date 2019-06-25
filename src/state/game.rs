use super::board::{Block, Board, Direction, Structure, Vertex};
use super::menu::{Menu, SidebarMenu};

pub struct GameState {
    pub active_menu: Option<Menu>,
    pub board: Board,
    pub focal_point: Vertex,
    pub player_mode: PlayerMode,
    pub highlighted_block: Option<Block>,
    pub highlighted_button: Option<SidebarMenu>,
    pub open_menu: Option<SidebarMenu>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            active_menu: None,
            board: Board::new(),
            focal_point: Vertex { x: 10, y: 20 },
            // player_mode: PlayerMode::Focus,
            player_mode: PlayerMode::PlaceStructure {
                structure: Structure::Forest,
                orientation: Direction::North,
            },
            // player_mode: PlayerMode::RaiseLower { radius: 0 },
            highlighted_block: None,
            highlighted_button: None,
            open_menu: None,
        }
    }

    pub fn selection_mode(&self) -> SelectionMode {
        match self.player_mode {
            PlayerMode::Focus => SelectionMode::None,
            PlayerMode::PlaceStructure {
                structure,
                orientation,
            } => {
                let (w, h) = structure.size();
                match orientation {
                    Direction::North | Direction::South => SelectionMode::Blocks { w, h },
                    Direction::East | Direction::West => SelectionMode::Blocks { w: h, h: w },
                }
            }
            PlayerMode::RaiseLower { radius } => SelectionMode::Vertex { radius },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerMode {
    Focus,
    RaiseLower {
        radius: u8,
    },
    PlaceStructure {
        structure: Structure,
        orientation: Direction,
    },
}

#[derive(PartialEq, Clone, Copy)]
pub enum SelectionMode {
    None,
    Vertex { radius: u8 },
    Blocks { w: u8, h: u8 },
}
