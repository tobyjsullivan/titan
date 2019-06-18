pub const BOARD_WIDTH: u32 = 1000;
pub const BOARD_HEIGHT: u32 = 1000;

pub const WATER_LEVEL: u8 = 0;

pub type ObjectDimension = u8;
pub type VertexHeight = u8;
pub type VertexPosition = u32;
pub type BlockPosition = u32;

pub struct GameBoard {
    vertices: [VertexHeight; ((BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)) as usize],
    objects: Vec<ObjectPlacement>,
}

impl GameBoard {
    fn new() -> Self {
        let mut res = Self {
            vertices: [WATER_LEVEL; ((BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)) as usize],
            objects: Vec::new(),
        };

        // Set a couple vertices to WATER+1
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
        ].iter() {
            res.vertices[v] += 1;
        }

        res
    }

    fn vertex_index(v: Vertex) -> usize {
        ((v.y * (BOARD_WIDTH + 1)) + v.x) as usize
    }

    pub fn vertex_height(&self, v: Vertex) -> VertexHeight {
        self.vertices[Self::vertex_index(v)]
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

pub struct Block {
    pub x: BlockPosition,
    pub y: BlockPosition,
}

pub struct Vertex {
    pub x: VertexPosition,
    pub y: VertexPosition,
}

pub enum LandType {
    Water,
    Land,
}

struct ObjectPlacement {
    object: Object,
    orientation: Direction,
    origin: Vertex,
}

pub enum Object {
    Forest,
}

impl Object {
    fn size(obj: Object) -> (ObjectDimension, ObjectDimension) {
        match obj {
            Object::Forest => (1, 1),
        }
    }
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub enum PlayerMode {
    Focus,
    RaiseLower { radius: u8 },
    PlaceObject { obj: Object, orientation: Direction },
}

pub struct GameState {
    pub board: GameBoard,
    pub mode: PlayerMode,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: GameBoard::new(),
            mode: PlayerMode::Focus,
        }
    }
}
