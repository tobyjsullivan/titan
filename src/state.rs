const BOARD_WIDTH: u32 = 100;
const BOARD_HEIGHT: u32 = 100;

pub const WATER_LEVEL: u8 = 0;

pub type StructureDimension = u8;
pub type VertexHeight = u8;
pub type VertexPosition = u32;
pub type BlockPosition = u32;

pub struct GameState {
    pub board: GameBoard,
    pub focal_point: Vertex,
    pub player_mode: PlayerMode,
    pub highlighted_block: Option<Block>,
    pub highlighted_button: Option<SidebarMenu>,
    pub open_menu: Option<SidebarMenu>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: GameBoard::new(),
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

pub struct GameBoard {
    vertices: [VertexHeight; ((BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)) as usize],
    structures: Vec<StructurePlacement>,
    block_occupants: [Option<usize>; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
}

impl GameBoard {
    fn new() -> Self {
        let mut res = Self {
            vertices: [WATER_LEVEL; ((BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)) as usize],
            structures: Vec::new(),
            block_occupants: [None; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
        };

        for y in 1..BOARD_HEIGHT {
            for x in 1..BOARD_WIDTH {
                res.vertices[Self::vertex_index(Vertex { x, y })] += 1;
            }
        }

        // Set a few vertices to WATER+1.
        for &v in [
            Self::vertex_index(Vertex { x: 5, y: 3 }),
            Self::vertex_index(Vertex { x: 5, y: 4 }),
            Self::vertex_index(Vertex { x: 5, y: 5 }),
            Self::vertex_index(Vertex { x: 5, y: 6 }),
            Self::vertex_index(Vertex { x: 6, y: 3 }),
            Self::vertex_index(Vertex { x: 6, y: 4 }),
            Self::vertex_index(Vertex { x: 6, y: 5 }),
            Self::vertex_index(Vertex { x: 6, y: 6 }),
            Self::vertex_index(Vertex { x: 7, y: 3 }),
            Self::vertex_index(Vertex { x: 7, y: 4 }),
            Self::vertex_index(Vertex { x: 7, y: 5 }),
            Self::vertex_index(Vertex { x: 7, y: 6 }),
            Self::vertex_index(Vertex { x: 8, y: 3 }),
            Self::vertex_index(Vertex { x: 8, y: 4 }),
            Self::vertex_index(Vertex { x: 8, y: 5 }),
            Self::vertex_index(Vertex { x: 8, y: 6 }),
            Self::vertex_index(Vertex { x: 9, y: 3 }),
            Self::vertex_index(Vertex { x: 9, y: 4 }),
            Self::vertex_index(Vertex { x: 9, y: 5 }),
            Self::vertex_index(Vertex { x: 9, y: 6 }),
            Self::vertex_index(Vertex { x: 7, y: 4 }),
            Self::vertex_index(Vertex { x: 7, y: 5 }),
            Self::vertex_index(Vertex { x: 8, y: 4 }),
            Self::vertex_index(Vertex { x: 8, y: 5 }),
        ]
        .iter()
        {
            res.vertices[v] += 1;
        }

        res
    }

    pub fn width(&self) -> u32 {
        BOARD_WIDTH
    }

    pub fn height(&self) -> u32 {
        BOARD_HEIGHT
    }

    pub fn vertex_on_board(&self, v: Vertex) -> bool {
        v.x <= BOARD_WIDTH && v.y <= BOARD_HEIGHT
    }

    fn vertex_index(v: Vertex) -> usize {
        ((v.y * (BOARD_WIDTH + 1)) + v.x) as usize
    }

    pub fn vertex_height(&self, v: Vertex) -> VertexHeight {
        self.vertices[Self::vertex_index(v)]
    }

    pub fn set_vertex_height(&mut self, v: Vertex, height: u8) {
        self.vertices[Self::vertex_index(v)] = height;
    }

    pub fn block_land_type(&self, x: VertexPosition, y: VertexPosition) -> LandType {
        for &h in [
            self.vertex_height(Vertex { x, y }),
            self.vertex_height(Vertex { x: x + 1, y }),
            self.vertex_height(Vertex { x: x + 1, y: y + 1 }),
            self.vertex_height(Vertex { x, y: y + 1 }),
        ]
        .iter()
        {
            if h > WATER_LEVEL {
                return LandType::Land;
            }
        }

        LandType::Water
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum SidebarMenu {
    Close,
    Save,
    Music,
    Graphics,
    Help,
    Rotation,
    Metrics,
    Finances,
    News,
    Info,
    Navigation,
    Building,
    Rail,
    Demolish,
    Point,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Block {
    pub x: BlockPosition,
    pub y: BlockPosition,
}

impl From<Vertex> for Block {
    fn from(v: Vertex) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vertex {
    pub x: VertexPosition,
    pub y: VertexPosition,
}

impl Vertex {
    pub fn is_edge_vertex(&self) -> bool {
        self.x == 0 || self.y == 0 || self.x == BOARD_WIDTH + 1 || self.y == BOARD_HEIGHT + 1
    }
}

impl From<Block> for Vertex {
    fn from(b: Block) -> Self {
        Self { x: b.x, y: b.y }
    }
}

pub enum LandType {
    Water,
    Land,
}

#[derive(PartialEq, Clone, Copy)]
struct StructurePlacement {
    structure: Structure,
    orientation: Direction,
    origin: Vertex,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Structure {
    Forest,
    RailPlatform,
}

impl Structure {
    fn size(&self) -> (StructureDimension, StructureDimension) {
        match self {
            Structure::Forest => (1, 1),
            Structure::RailPlatform => (4, 2),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
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
