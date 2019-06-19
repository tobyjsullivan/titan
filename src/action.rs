use crate::state::Block;

pub enum GameAction {
    Hover { block: Option<Block> },
    Focus,
    RaiseTerrain,
    LowerTerrain,
}
