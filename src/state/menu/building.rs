#[derive(PartialEq, Copy, Clone)]
pub enum BuyBuildingScreenState {
    Visible {
        selected_building: Building,
        selected_category: Category,
    },
    Hidden,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Category {
    Terminals,
    Production,
    Retail,
    CityBuildings,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Building {
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
