use crate::state::{Block, SidebarButton};

pub enum GameAction {
    Hover { block: Option<Block> },
    SidebarHover { button: Option<SidebarButton> },
    Focus,
    RaiseTerrain,
    LowerTerrain,
}
