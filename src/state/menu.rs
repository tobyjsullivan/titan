pub enum Menu {
    BuyBuilding {
        selected_category: BuyBuildingCategory,
        selected_building: BuyBuildingBuilding,
    },
}

pub enum BuyBuildingCategory {
    Terminals,
    Production,
    Retail,
    CityBuildings,
}

pub enum BuyBuildingBuilding {
    // Terminals
    TruckDepot,
    TrainStation,
    Airport,
    Harbor,

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

    // City Buildings
    CulturalCenter,
    TennisCourt,
    SwimmingPool,
    SportsStadium,
    RaceTrack,
    University,
    AmusementPark,
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
