pub const BOARD_WIDTH: u32 = 1000;
pub const BOARD_HEIGHT: u32 = 1000;

const WATER_LEVEL: u8 = 0;

pub type ObjectDimension = u8;
pub type VertexHeight = u8;
pub type VertexPosition = u32;

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
        let v1 = Self::vertex_index(Vertex { x: 5, y: 3 });
        let v2 = Self::vertex_index(Vertex { x: 13, y: 8 });
        res.vertices[v1] = WATER_LEVEL + 1;
        res.vertices[v2] = WATER_LEVEL + 1;

        res
    }

    fn vertex_index(v: Vertex) -> usize {
        ((v.y * (BOARD_WIDTH + 1)) + v.x) as usize
    }

    pub fn vertex_height(&self, v: Vertex) -> VertexHeight {
        self.vertices[Self::vertex_index(v)]
    }

    fn block_info(&self, x: VertexPosition, y: VertexPosition) -> BlockInfo {
        BlockInfo {
            nw_height: self.vertex_height(Vertex { x, y }),
            ne_height: self.vertex_height(Vertex { x: x + 1, y }),
            se_height: self.vertex_height(Vertex { x: x + 1, y: y + 1 }),
            sw_height: self.vertex_height(Vertex { x, y: y + 1 }),
        }
    }
}

pub struct Vertex {
    pub x: VertexPosition,
    pub y: VertexPosition,
}

struct BlockInfo {
    nw_height: VertexHeight,
    ne_height: VertexHeight,
    sw_height: VertexHeight,
    se_height: VertexHeight,
}

impl BlockInfo {
    fn is_flat(&self) -> bool {
        (self.nw_height == self.ne_height
            && self.nw_height == self.sw_height
            && self.nw_height == self.se_height)
    }
}

struct ObjectPlacement {
    object: Object,
    orientation: Direction,
    origin: Vertex,
}

enum Object {
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

pub struct GameState {
    pub board: GameBoard,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: GameBoard::new(),
        }
    }
}
