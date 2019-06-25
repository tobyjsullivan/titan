use crate::state::board::Block;
use crate::state::menu::sidebar::SidebarMenu;

pub enum GameAction {
    Hover { block: Option<Block> },
    SidebarHover { button: Option<SidebarMenu> },
    OpenMenu { menu: SidebarMenu },
    PlaceStructure,
    Focus,
    RaiseTerrain,
    RotateStructure,
    LowerTerrain,
}
