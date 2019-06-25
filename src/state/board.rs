const BOARD_WIDTH: u32 = 100;
const BOARD_HEIGHT: u32 = 100;

pub const WATER_LEVEL: u8 = 0;

pub type StructureDimension = u8;
pub type VertexHeight = u8;
pub type VertexPosition = u32;
pub type BlockPosition = u32;

pub struct Board {
    vertices: [VertexHeight; ((BOARD_WIDTH + 1) * (BOARD_HEIGHT + 1)) as usize],
    structures: Vec<StructurePlacement>,
    block_occupants: [Option<usize>; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
}

impl Board {
    pub fn new() -> Self {
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

    fn block_index(b: Block) -> usize {
        ((b.y * BOARD_WIDTH) + b.x) as usize
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

    pub fn block_structure_type(&self, block: Block) -> Option<Structure> {
        match self.block_occupants[Self::block_index(block)] {
            Some(idx) => Some(self.structures[idx].structure),
            None => None,
        }
    }

    pub fn place_structure(&mut self, structure: Structure, orientation: Direction, origin: Block) {
        let idx = self.structures.len();
        let placement = StructurePlacement {
            structure,
            orientation,
            origin,
        };
        self.structures.push(placement);
        // Iterate over each block the structure covers and insert the index.
        // TODO (toby): Pre-check if any block is already occupied and return a result.
        let w = placement.width();
        let h = placement.height();
        for y in origin.y..origin.y + h as u32 {
            for x in origin.x..origin.x + w as u32 {
                self.block_occupants[Self::block_index(Block { x, y })] = Some(idx);
            }
        }
    }
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
    origin: Block,
}

impl StructurePlacement {
    fn width(&self) -> StructureDimension {
        let (w, h) = self.structure.size();
        match self.orientation {
            Direction::North | Direction::South => w,
            Direction::East | Direction::West => h,
        }
    }

    fn height(&self) -> StructureDimension {
        let (w, h) = self.structure.size();
        match self.orientation {
            Direction::North | Direction::South => h,
            Direction::East | Direction::West => w,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Mineral {
    Gold,
    Silver,
    Diamonds,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Structure {
    // Nature
    Forest,

    // City
    CityRoad,
    ApartmentBuilding, // TODO (toby): There should probably be many city building types.

    // City (Player Built)
    CulturalCenter,
    TennisCourt,
    SwimmingPool,
    SportsStadium,
    RaceTrack,
    University,
    AmusementPark,

    // Resources
    LumberMill,
    ChemicalPlant,
    SteelMill,
    Mine { mineral: Mineral },

    // Transportation
    Street,
    Rails,
    Bridge,
    Tunnel,

    // Terminals
    TruckDepot,
    TrainStation,
    TrainPlatform,
    Harbor,
    Airport,

    // Production
    AutomobileFactory,
    Woodshop,
    ElectronicsFactory,
    SportsEquipmentFactory,
    ToyFactory,
    JewelryFactory,
    Warehouse,
    BuildingEquipmentFactory,
    PaperFactory,
    PrintingPress,

    // Retail
    ToyStore,
    SportingGoodsStore,
    FurnitureStore,
    Jeweler,
    ElectronicsStore,
    CarDealership,
    BuildingEquipmentStore,
    StationaryStore,
}

impl Structure {
    pub fn size(&self) -> (StructureDimension, StructureDimension) {
        match self {
            Structure::Forest => (1, 1),
            Structure::CityRoad => (1, 1),
            Structure::ApartmentBuilding => (1, 1),
            Structure::CulturalCenter => (1, 1),
            Structure::TennisCourt => (2, 2),
            Structure::SwimmingPool => (2, 2),
            Structure::SportsStadium => (2, 3),
            Structure::RaceTrack => (3, 3),
            Structure::University => (3, 3),
            Structure::AmusementPark => (5, 5),
            Structure::LumberMill => (4, 4),
            Structure::ChemicalPlant => (5, 5),
            Structure::SteelMill => (5, 5),
            Structure::Mine { .. } => (2, 2),
            Structure::Street => (1, 1),
            Structure::Rails => (1, 1),
            Structure::Bridge => (1, 1),
            Structure::Tunnel => (1, 1),
            Structure::TruckDepot => (1, 1),
            Structure::TrainStation => (4, 2),
            Structure::TrainPlatform => (4, 1),
            Structure::Harbor => (3, 3),
            Structure::Airport => (5, 5),
            Structure::AutomobileFactory => (5, 5),
            Structure::Woodshop => (4, 4),
            Structure::ElectronicsFactory => (4, 4),
            Structure::SportsEquipmentFactory => (4, 4),
            Structure::ToyFactory => (4, 4),
            Structure::JewelryFactory => (3, 3),
            Structure::Warehouse => (4, 4),
            Structure::BuildingEquipmentFactory => (4, 4),
            Structure::PaperFactory => (4, 4),
            Structure::PrintingPress => (4, 4),
            Structure::ToyStore => (2, 2),
            Structure::SportingGoodsStore => (2, 2),
            Structure::FurnitureStore => (2, 2),
            Structure::Jeweler => (2, 2),
            Structure::ElectronicsStore => (2, 2),
            Structure::CarDealership => (2, 2),
            Structure::BuildingEquipmentStore => (3, 3),
            Structure::StationaryStore => (2, 2),
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
