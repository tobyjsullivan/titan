use crate::state::{Block, PlayerMode, SidebarButton};

pub enum GameAction {
    Hover { block: Option<Block> },
    SidebarHover { button: Option<SidebarButton> },
    SetPlayerMode { mode: PlayerMode },
    Focus,
    RaiseTerrain,
    LowerTerrain,
}
