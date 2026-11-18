#[derive(Clone, Copy, PartialEq)]
pub enum ItemKind {
    AmmoRefill,
    Grenade,
    Landmine,
    Claymore,
    MedKit,
    ArmorPlate,
    Nuke,
}

pub struct ShopItem {
    pub name: &'static str,
    pub desc: &'static str,
    pub price: u32,
    pub kind: ItemKind,
}

pub const INGAME_SHOP: [ShopItem; 7] = [
    ShopItem {
        name: "AMMO REFILL",
        desc: "Full reserve for current weapon",
        price: 100,
        kind: ItemKind::AmmoRefill,
    },
    ShopItem {
        name: "GRENADE",
        desc: "+1 throwable grenade",
        price: 150,
        kind: ItemKind::Grenade,
    },
    ShopItem {
        name: "MED KIT",
        desc: "+50 health",
        price: 200,
        kind: ItemKind::MedKit,
    },
    ShopItem {
        name: "ARMOR PLATE",
        desc: "+25 armor (max 50)",
        price: 250,
        kind: ItemKind::ArmorPlate,
    },
    ShopItem {
        name: "LANDMINE",
        desc: "+1 proximity mine (4m blast)",
        price: 250,
        kind: ItemKind::Landmine,
    },
    ShopItem {
        name: "CLAYMORE",
        desc: "+1 directional mine (6m blast)",
        price: 400,
        kind: ItemKind::Claymore,
    },
    ShopItem {
        name: "NUKE",
        desc: "Wipes ALL zombies on the map",
        price: 10_000,
        kind: ItemKind::Nuke,
    },
];
