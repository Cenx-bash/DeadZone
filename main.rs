use macroquad::prelude::*;

// ===================== CONSTANTS =====================
const HALF: f32 = 24.0;
const EYE_STAND: f32 = 1.7;
const EYE_CROUCH: f32 = 1.0;
const PLAYER_R: f32 = 0.5;
const SENS: f32 = 0.0035;
const PILLAR_R: f32 = 1.1;
const GRAVITY: f32 = 22.0;
const JUMP_VEL: f32 = 7.5;
const WALK_SPEED: f32 = 6.0;
const SPRINT_SPEED: f32 = 9.5;
const CROUCH_SPEED: f32 = 3.2;
const STAMINA_MAX: f32 = 100.0;
const STAMINA_DRAIN: f32 = 24.0;
const STAMINA_REGEN: f32 = 20.0;
const SAVE_PATH: &str = "fps_save.dat";

// ===================== ENUMS =====================
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Loading,
    Menu,
    Shop,
    Inventory,
    Abilities,
    Play,
    Pause,
    Dead,
}

#[derive(Clone, Copy, PartialEq)]
enum MenuCmd {
    Play,
    Shop,
    Inventory,
    Abilities,
    CycleMap,
    Quit,
}

#[derive(Clone, Copy, PartialEq)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    fn color(self) -> Color {
        match self {
            Rarity::Common => Color::new(0.70, 0.70, 0.70, 1.0),
            Rarity::Uncommon => Color::new(0.30, 0.90, 0.40, 1.0),
            Rarity::Rare => Color::new(0.30, 0.60, 1.00, 1.0),
            Rarity::Epic => Color::new(0.70, 0.40, 1.00, 1.0),
            Rarity::Legendary => Color::new(1.00, 0.80, 0.20, 1.0),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Rarity::Common => "COMMON",
            Rarity::Uncommon => "UNCOMMON",
            Rarity::Rare => "RARE",
            Rarity::Epic => "EPIC",
            Rarity::Legendary => "LEGENDARY",
        }
    }
}

// ===================== WEAPONS =====================
#[derive(Clone, Copy)]
struct WeaponDef {
    name: &'static str,
    damage: f32,
    fire_rate: f32,
    reload_time: f32,
    mag_size: u32,
    recoil: f32,
    spread: f32,
    range: f32,
    auto: bool,
    pellets: u32,
    price: u32,
    rarity: Rarity,
    color: Color,
    model_len: usize,
    model_radius: f32,
    is_melee: bool,
}

const WEAPONS: [WeaponDef; 9] = [
    // 0: PISTOL (starting)
    WeaponDef {
        name: "PISTOL",
        damage: 20.0,
        fire_rate: 6.0,
        reload_time: 1.1,
        mag_size: 15,
        recoil: 0.6,
        spread: 0.002,
        range: 60.0,
        auto: false,
        pellets: 1,
        price: 0,
        rarity: Rarity::Common,
        color: Color::new(0.75, 0.78, 0.85, 1.0),
        model_len: 3,
        model_radius: 0.05,
        is_melee: false,
    },
    // 1: KNIFE (always owned — "2nd weapon")
    WeaponDef {
        name: "KNIFE",
        damage: 120.0,
        fire_rate: 2.2,
        reload_time: 0.0,
        mag_size: 0,
        recoil: 0.0,
        spread: 0.0,
        range: 2.5,
        auto: true,
        pellets: 1,
        price: 0,
        rarity: Rarity::Common,
        color: Color::new(0.85, 0.85, 0.92, 1.0),
        model_len: 1,
        model_radius: 0.03,
        is_melee: true,
    },
    // 2: SMG
    WeaponDef {
        name: "SMG",
        damage: 14.0,
        fire_rate: 13.0,
        reload_time: 1.6,
        mag_size: 35,
        recoil: 1.0,
        spread: 0.020,
        range: 45.0,
        auto: true,
        pellets: 1,
        price: 400,
        rarity: Rarity::Uncommon,
        color: Color::new(0.50, 0.75, 1.00, 1.0),
        model_len: 5,
        model_radius: 0.05,
        is_melee: false,
    },
    // 3: SHOTGUN
    WeaponDef {
        name: "SHOTGUN",
        damage: 12.0,
        fire_rate: 1.4,
        reload_time: 2.2,
        mag_size: 6,
        recoil: 2.2,
        spread: 0.090,
        range: 22.0,
        auto: false,
        pellets: 8,
        price: 900,
        rarity: Rarity::Rare,
        color: Color::new(1.00, 0.62, 0.30, 1.0),
        model_len: 6,
        model_radius: 0.07,
        is_melee: false,
    },
    // 4: RIFLE
    WeaponDef {
        name: "RIFLE",
        damage: 42.0,
        fire_rate: 3.0,
        reload_time: 1.8,
        mag_size: 25,
        recoil: 1.4,
        spread: 0.005,
        range: 80.0,
        auto: false,
        pellets: 1,
        price: 1400,
        rarity: Rarity::Rare,
        color: Color::new(0.55, 0.95, 0.55, 1.0),
        model_len: 7,
        model_radius: 0.045,
        is_melee: false,
    },
    // 5: SNIPER
    WeaponDef {
        name: "SNIPER",
        damage: 180.0,
        fire_rate: 0.9,
        reload_time: 2.5,
        mag_size: 5,
        recoil: 3.5,
        spread: 0.0005,
        range: 200.0,
        auto: false,
        pellets: 1,
        price: 2200,
        rarity: Rarity::Epic,
        color: Color::new(0.70, 0.40, 1.00, 1.0),
        model_len: 10,
        model_radius: 0.04,
        is_melee: false,
    },
    // 6: LMG
    WeaponDef {
        name: "LMG",
        damage: 28.0,
        fire_rate: 10.0,
        reload_time: 3.5,
        mag_size: 100,
        recoil: 1.8,
        spread: 0.014,
        range: 70.0,
        auto: true,
        pellets: 1,
        price: 3200,
        rarity: Rarity::Legendary,
        color: Color::new(1.00, 0.82, 0.20, 1.0),
        model_len: 9,
        model_radius: 0.06,
        is_melee: false,
    },
    // 7: REVOLVER (NEW)
    WeaponDef {
        name: "REVOLVER",
        damage: 70.0,
        fire_rate: 2.0,
        reload_time: 2.0,
        mag_size: 6,
        recoil: 2.5,
        spread: 0.003,
        range: 70.0,
        auto: false,
        pellets: 1,
        price: 800,
        rarity: Rarity::Uncommon,
        color: Color::new(0.75, 0.55, 0.35, 1.0),
        model_len: 4,
        model_radius: 0.055,
        is_melee: false,
    },
    // 8: DMR (NEW)
    WeaponDef {
        name: "DMR",
        damage: 55.0,
        fire_rate: 4.0,
        reload_time: 2.0,
        mag_size: 15,
        recoil: 1.8,
        spread: 0.003,
        range: 120.0,
        auto: false,
        pellets: 1,
        price: 1800,
        rarity: Rarity::Epic,
        color: Color::new(0.50, 0.90, 0.70, 1.0),
        model_len: 8,
        model_radius: 0.045,
        is_melee: false,
    },
];

// ===================== ATTACHMENTS =====================
#[derive(Clone, Copy)]
struct AttachmentDef {
    name: &'static str,
    slot: usize,
    damage_mult: f32,
    spread_mult: f32,
    recoil_mult: f32,
    reload_mult: f32,
    mag_add: i32,
    zoom: f32,
    price: u32,
    desc: &'static str,
}

const ATTACHMENTS: [AttachmentDef; 13] = [
    AttachmentDef {
        name: "RED DOT",
        slot: 0,
        damage_mult: 1.00,
        spread_mult: 0.85,
        recoil_mult: 1.00,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.15,
        price: 150,
        desc: "+accuracy",
    },
    AttachmentDef {
        name: "HOLO",
        slot: 0,
        damage_mult: 1.00,
        spread_mult: 0.80,
        recoil_mult: 1.00,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.30,
        price: 300,
        desc: "++accuracy",
    },
    AttachmentDef {
        name: "ACOG",
        slot: 0,
        damage_mult: 1.00,
        spread_mult: 0.75,
        recoil_mult: 1.00,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.65,
        price: 550,
        desc: "+++accuracy",
    },
    AttachmentDef {
        name: "SNIPER SCOPE",
        slot: 0,
        damage_mult: 1.00,
        spread_mult: 0.70,
        recoil_mult: 1.00,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 2.60,
        price: 900,
        desc: "high zoom",
    },
    AttachmentDef {
        name: "SUPPRESSOR",
        slot: 1,
        damage_mult: 0.95,
        spread_mult: 0.90,
        recoil_mult: 0.80,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 350,
        desc: "-recoil -dmg",
    },
    AttachmentDef {
        name: "COMPENSATOR",
        slot: 1,
        damage_mult: 1.00,
        spread_mult: 0.85,
        recoil_mult: 0.75,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 300,
        desc: "--recoil",
    },
    AttachmentDef {
        name: "EXT BARREL",
        slot: 1,
        damage_mult: 1.15,
        spread_mult: 0.90,
        recoil_mult: 1.00,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 600,
        desc: "+damage",
    },
    AttachmentDef {
        name: "VERT GRIP",
        slot: 2,
        damage_mult: 1.00,
        spread_mult: 1.00,
        recoil_mult: 0.70,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 250,
        desc: "--recoil",
    },
    AttachmentDef {
        name: "ANGLED GRIP",
        slot: 2,
        damage_mult: 1.00,
        spread_mult: 0.90,
        recoil_mult: 0.85,
        reload_mult: 0.92,
        mag_add: 0,
        zoom: 1.00,
        price: 300,
        desc: "-recoil +reload",
    },
    AttachmentDef {
        name: "EXT MAG",
        slot: 3,
        damage_mult: 1.00,
        spread_mult: 1.00,
        recoil_mult: 1.00,
        reload_mult: 1.05,
        mag_add: 15,
        zoom: 1.00,
        price: 350,
        desc: "+15 mag",
    },
    AttachmentDef {
        name: "FAST MAG",
        slot: 3,
        damage_mult: 1.00,
        spread_mult: 1.00,
        recoil_mult: 1.00,
        reload_mult: 0.70,
        mag_add: 0,
        zoom: 1.00,
        price: 300,
        desc: "-30% reload",
    },
    AttachmentDef {
        name: "TACTICAL STOCK",
        slot: 4,
        damage_mult: 1.00,
        spread_mult: 0.80,
        recoil_mult: 0.88,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 350,
        desc: "-spread -recoil",
    },
    AttachmentDef {
        name: "LIGHTWEIGHT",
        slot: 4,
        damage_mult: 1.00,
        spread_mult: 1.05,
        recoil_mult: 1.00,
        reload_mult: 0.88,
        mag_add: 0,
        zoom: 1.00,
        price: 200,
        desc: "+reload",
    },
];

const SLOT_NAMES: [&str; 5] = ["OPTIC", "BARREL", "UNDERBARREL", "MAGAZINE", "STOCK"];

// ===================== ABILITIES =====================
struct AbilityDef {
    name: &'static str,
    desc: &'static str,
    max_level: u32,
    base_cost: u32,
}

const ABILITIES: [AbilityDef; 10] = [
    AbilityDef {
        name: "SPEED BOOST",
        desc: "+8% move speed / lvl",
        max_level: 5,
        base_cost: 200,
    },
    AbilityDef {
        name: "DOUBLE JUMP",
        desc: "Jump again in midair",
        max_level: 1,
        base_cost: 800,
    },
    AbilityDef {
        name: "QUICK RELOAD",
        desc: "-10% reload / lvl",
        max_level: 5,
        base_cost: 250,
    },
    AbilityDef {
        name: "EXTRA HEALTH",
        desc: "+20 max HP / lvl",
        max_level: 5,
        base_cost: 300,
    },
    AbilityDef {
        name: "REGENERATION",
        desc: "+1 HP/sec / lvl",
        max_level: 4,
        base_cost: 400,
    },
    AbilityDef {
        name: "DAMAGE BOOST",
        desc: "+10% damage / lvl",
        max_level: 5,
        base_cost: 350,
    },
    AbilityDef {
        name: "FAST SPRINT",
        desc: "+10% sprint / lvl",
        max_level: 3,
        base_cost: 250,
    },
    AbilityDef {
        name: "AMMO SCAVENGER",
        desc: "+25% pickup ammo / lvl",
        max_level: 4,
        base_cost: 200,
    },
    AbilityDef {
        name: "CRITICAL SHOT",
        desc: "+20% headshot dmg / lvl",
        max_level: 5,
        base_cost: 400,
    },
    AbilityDef {
        name: "EXPLOSIVE ROUNDS",
        desc: "AoE damage on hit",
        max_level: 3,
        base_cost: 600,
    },
];

fn ability_cost(idx: usize, level: u32) -> u32 {
    ABILITIES[idx].base_cost * (level + 1) * (level + 1)
}

// ===================== SAVE =====================
#[derive(Clone)]
struct Save {
    coins: u32,
    owned_weapons: [bool; 9],
    owned_attachments: [bool; 13],
    ability_levels: [u32; 10],
    equipped_weapon: usize,
    weapon_attachments: [[i32; 5]; 9],
    best_wave: u32,
    total_kills: u32,
}

impl Save {
    fn fresh() -> Self {
        let mut owned = [false; 9];
        owned[0] = true; // pistol
        owned[1] = true; // knife (always owned)
        Self {
            coins: 0,
            owned_weapons: owned,
            owned_attachments: [false; 13],
            ability_levels: [0; 10],
            equipped_weapon: 0,
            weapon_attachments: [[-1; 5]; 9],
            best_wave: 0,
            total_kills: 0,
        }
    }

    fn save_to_disk(&self) {
        let mut s = String::new();
        s.push_str(&format!("coins={}\n", self.coins));
        s.push_str(&format!("equipped={}\n", self.equipped_weapon));
        s.push_str(&format!("best_wave={}\n", self.best_wave));
        s.push_str(&format!("total_kills={}\n", self.total_kills));
        s.push_str("weapons=");
        for (i, o) in self.owned_weapons.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(if *o { "1" } else { "0" });
        }
        s.push('\n');
        s.push_str("attachments=");
        for (i, o) in self.owned_attachments.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(if *o { "1" } else { "0" });
        }
        s.push('\n');
        s.push_str("abilities=");
        for (i, l) in self.ability_levels.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&l.to_string());
        }
        s.push('\n');
        for w in 0..9 {
            s.push_str(&format!("watt_{}=", w));
            for sl in 0..5 {
                if sl > 0 {
                    s.push(',');
                }
                s.push_str(&self.weapon_attachments[w][sl].to_string());
            }
            s.push('\n');
        }
        let _ = std::fs::write(SAVE_PATH, s.as_bytes());
    }

    fn load_from_disk() -> Self {
        let mut out = Save::fresh();
        let data = match std::fs::read(SAVE_PATH) {
            Ok(d) => d,
            Err(_) => return out,
        };
        let text = String::from_utf8_lossy(&data);
        for line in text.lines() {
            let mut it = line.splitn(2, '=');
            let key = it.next().unwrap_or("");
            let val = it.next().unwrap_or("");
            match key {
                "coins" => {
                    if let Ok(v) = val.parse() {
                        out.coins = v;
                    }
                }
                "equipped" => {
                    if let Ok(v) = val.parse::<usize>() {
                        if v < 9 {
                            out.equipped_weapon = v;
                        }
                    }
                }
                "best_wave" => {
                    if let Ok(v) = val.parse() {
                        out.best_wave = v;
                    }
                }
                "total_kills" => {
                    if let Ok(v) = val.parse() {
                        out.total_kills = v;
                    }
                }
                "weapons" => {
                    for (i, p) in val.split(',').enumerate() {
                        if i < 9 {
                            out.owned_weapons[i] = p == "1";
                        }
                    }
                }
                "attachments" => {
                    for (i, p) in val.split(',').enumerate() {
                        if i < 13 {
                            out.owned_attachments[i] = p == "1";
                        }
                    }
                }
                "abilities" => {
                    for (i, p) in val.split(',').enumerate() {
                        if i < 10 {
                            if let Ok(v) = p.parse() {
                                out.ability_levels[i] = v;
                            }
                        }
                    }
                }
                _ => {
                    if let Some(rest) = key.strip_prefix("watt_") {
                        if let Ok(w) = rest.parse::<usize>() {
                            if w < 9 {
                                for (sl, p) in val.split(',').enumerate() {
                                    if sl < 5 {
                                        if let Ok(v) = p.parse::<i32>() {
                                            out.weapon_attachments[w][sl] = v;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Guarantee starting weapons
        out.owned_weapons[0] = true;
        out.owned_weapons[1] = true;
        out
    }
}

// ===================== ZOMBIES =====================
#[derive(Clone, Copy, PartialEq)]
enum ZombieKind {
    Walker,
    Runner,
    Tank,
    Crawler,
    Spitter,
    Boss,
}

impl ZombieKind {
    fn hp(self, wave: u32) -> f32 {
        let base = match self {
            ZombieKind::Walker => 30.0,
            ZombieKind::Runner => 22.0,
            ZombieKind::Tank => 180.0,
            ZombieKind::Crawler => 18.0,
            ZombieKind::Spitter => 45.0,
            ZombieKind::Boss => 900.0,
        };
        base * (1.0 + wave as f32 * 0.06)
    }
    fn speed(self) -> f32 {
        match self {
            ZombieKind::Walker => 2.2,
            ZombieKind::Runner => 5.8,
            ZombieKind::Tank => 1.6,
            ZombieKind::Crawler => 3.6,
            ZombieKind::Spitter => 2.0,
            ZombieKind::Boss => 2.6,
        }
    }
    fn radius(self) -> f32 {
        match self {
            ZombieKind::Walker => 0.5,
            ZombieKind::Runner => 0.4,
            ZombieKind::Tank => 0.9,
            ZombieKind::Crawler => 0.55,
            ZombieKind::Spitter => 0.5,
            ZombieKind::Boss => 1.4,
        }
    }
    fn height(self) -> f32 {
        match self {
            ZombieKind::Walker => 1.8,
            ZombieKind::Runner => 1.4,
            ZombieKind::Tank => 2.8,
            ZombieKind::Crawler => 0.7,
            ZombieKind::Spitter => 1.9,
            ZombieKind::Boss => 4.2,
        }
    }
    fn damage(self) -> f32 {
        match self {
            ZombieKind::Walker => 10.0,
            ZombieKind::Runner => 8.0,
            ZombieKind::Tank => 28.0,
            ZombieKind::Crawler => 6.0,
            ZombieKind::Spitter => 0.0,
            ZombieKind::Boss => 45.0,
        }
    }
    fn score(self) -> u32 {
        match self {
            ZombieKind::Walker => 10,
            ZombieKind::Runner => 15,
            ZombieKind::Tank => 60,
            ZombieKind::Crawler => 12,
            ZombieKind::Spitter => 25,
            ZombieKind::Boss => 500,
        }
    }
    fn color(self) -> Color {
        match self {
            ZombieKind::Walker => Color::new(0.55, 0.45, 0.35, 1.0),
            ZombieKind::Runner => Color::new(0.85, 0.45, 0.25, 1.0),
            ZombieKind::Tank => Color::new(0.45, 0.25, 0.50, 1.0),
            ZombieKind::Crawler => Color::new(0.50, 0.55, 0.30, 1.0),
            ZombieKind::Spitter => Color::new(0.40, 0.80, 0.40, 1.0),
            ZombieKind::Boss => Color::new(0.85, 0.15, 0.15, 1.0),
        }
    }
}

// ===================== ENTITIES =====================
struct Pillar {
    x: f32,
    z: f32,
}

struct Enemy {
    pos: Vec3,
    kind: ZombieKind,
    hp: f32,
    max_hp: f32,
    flash: f32,
    atk_cd: f32,
    phase: f32,
    shoot_cd: f32,
    is_boss: bool,
}

struct Tracer {
    a: Vec3,
    b: Vec3,
    life: f32,
}
struct Particle {
    pos: Vec3,
    vel: Vec3,
    life: f32,
    max_life: f32,
    color: Color,
    size: f32,
}
struct Shell {
    pos: Vec3,
    vel: Vec3,
    life: f32,
}
struct FloatingText {
    pos: Vec3,
    text: String,
    life: f32,
    color: Color,
}
struct Projectile {
    pos: Vec3,
    vel: Vec3,
    life: f32,
    damage: f32,
}
struct Grenade {
    pos: Vec3,
    vel: Vec3,
    fuse: f32,
}

// ===================== MAP =====================
struct MapDef {
    name: &'static str,
    pillars: Vec<(f32, f32)>,
    floor: Color,
    wall: Color,
    accent: Color,
}

fn maps() -> Vec<MapDef> {
    vec![
        MapDef {
            name: "ARENA",
            pillars: vec![
                (-8.0, -8.0),
                (8.0, -8.0),
                (-8.0, 8.0),
                (8.0, 8.0),
                (0.0, -12.0),
                (0.0, 12.0),
                (-13.0, 0.0),
                (13.0, 0.0),
            ],
            floor: Color::new(0.12, 0.13, 0.18, 1.0),
            wall: Color::new(0.27, 0.30, 0.40, 1.0),
            accent: Color::new(0.60, 0.65, 0.85, 1.0),
        },
        MapDef {
            name: "CROSSFIRE",
            pillars: vec![
                (0.0, -9.0),
                (0.0, 9.0),
                (-9.0, 0.0),
                (9.0, 0.0),
                (0.0, 0.0),
                (-6.0, -6.0),
                (6.0, -6.0),
                (-6.0, 6.0),
                (6.0, 6.0),
            ],
            floor: Color::new(0.10, 0.15, 0.13, 1.0),
            wall: Color::new(0.23, 0.36, 0.30, 1.0),
            accent: Color::new(0.40, 0.80, 0.60, 1.0),
        },
        MapDef {
            name: "FORTRESS",
            pillars: vec![
                (-14.0, -14.0),
                (14.0, -14.0),
                (-14.0, 14.0),
                (14.0, 14.0),
                (0.0, 0.0),
                (-7.0, 0.0),
                (7.0, 0.0),
                (0.0, -7.0),
                (0.0, 7.0),
            ],
            floor: Color::new(0.16, 0.12, 0.13, 1.0),
            wall: Color::new(0.44, 0.27, 0.28, 1.0),
            accent: Color::new(0.85, 0.50, 0.50, 1.0),
        },
        MapDef {
            name: "WASTELAND",
            pillars: vec![
                (-12.0, -5.0),
                (12.0, 5.0),
                (-5.0, 10.0),
                (5.0, -10.0),
                (0.0, 0.0),
                (-15.0, 8.0),
                (15.0, -8.0),
            ],
            floor: Color::new(0.18, 0.16, 0.12, 1.0),
            wall: Color::new(0.40, 0.35, 0.22, 1.0),
            accent: Color::new(0.85, 0.75, 0.45, 1.0),
        },
    ]
}

// ===================== HELPERS =====================
fn push_out(pos: &mut Vec3, r: f32, pillars: &[Pillar]) {
    for p in pillars {
        let d = vec2(pos.x - p.x, pos.z - p.z);
        let min = PILLAR_R + r;
        let len = d.length();
        if len < min {
            let n = if len > 0.001 { d / len } else { vec2(1.0, 0.0) };
            pos.x = p.x + n.x * min;
            pos.z = p.z + n.y * min;
        }
    }
    let lim = HALF - r;
    pos.x = pos.x.clamp(-lim, lim);
    pos.z = pos.z.clamp(-lim, lim);
}

fn look_dir(yaw: f32, pitch: f32) -> Vec3 {
    vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize()
}

// ===================== GAME =====================
struct Game {
    pos: Vec3,
    vel_y: f32,
    yaw: f32,
    pitch: f32,
    grounded: bool,
    double_jump_used: bool,
    crouching: bool,
    stamina: f32,
    health: f32,
    max_health: f32,
    armor: f32,
    bob: f32,
    hurt_flash: f32,
    shake: f32,
    equipped: usize,
    ammo: u32,
    reserve: u32,
    reload_t: f32,
    cooldown: f32,
    recoil: f32,
    ads: bool,
    coins: u32,
    ability_lv: [u32; 10],
    grenade_count: u32,
    grenade_cd: f32,
    grenades: Vec<Grenade>,
    pillars: Vec<Pillar>,
    floor: Color,
    wall: Color,
    accent: Color,
    map_name: &'static str,
    enemies: Vec<Enemy>,
    tracers: Vec<Tracer>,
    particles: Vec<Particle>,
    shells: Vec<Shell>,
    floaters: Vec<FloatingText>,
    projectiles: Vec<Projectile>,
    wave: u32,
    to_spawn: u32,
    spawn_t: f32,
    wave_break: f32,
    kills: u32,
    score: u32,
    hit_mark: f32,
    muzzle_flash: f32,
    recoil_mult: f32,
    spread_mult: f32,
    reload_mult: f32,
    damage_mult: f32,
    mag_bonus: i32,
    zoom_mult: f32,
}

fn compute_mods(save: &Save, w_idx: usize) -> (f32, f32, f32, f32, i32, f32) {
    let mut recoil_mult: f32 = 1.0;
    let mut spread_mult: f32 = 1.0;
    let mut reload_mult: f32 = 1.0;
    let mut damage_mult: f32 = 1.0;
    let mut mag_bonus: i32 = 0;
    let mut zoom_mult: f32 = 1.0;
    for slot in 0..5 {
        let ai = save.weapon_attachments[w_idx][slot];
        if ai >= 0 && (ai as usize) < ATTACHMENTS.len() {
            let a = &ATTACHMENTS[ai as usize];
            recoil_mult *= a.recoil_mult;
            spread_mult *= a.spread_mult;
            reload_mult *= a.reload_mult;
            damage_mult *= a.damage_mult;
            mag_bonus += a.mag_add;
            zoom_mult = zoom_mult.max(a.zoom);
        }
    }
    (
        recoil_mult,
        spread_mult,
        reload_mult,
        damage_mult,
        mag_bonus,
        zoom_mult,
    )
}

impl Game {
    fn new(save: &Save, map_idx: usize) -> Self {
        let mdefs = maps();
        let def = &mdefs[map_idx % mdefs.len()];
        let pillars: Vec<Pillar> = def.pillars.iter().map(|&(x, z)| Pillar { x, z }).collect();

        let w = save.equipped_weapon.min(8);
        let (recoil_mult, spread_mult, reload_mult, damage_mult, mag_bonus, zoom_mult) =
            compute_mods(save, w);

        let max_hp = 100.0 + save.ability_levels[3] as f32 * 20.0;
        let wdef = WEAPONS[w];
        let mag = if wdef.is_melee {
            0
        } else {
            ((wdef.mag_size as i32 + mag_bonus).max(1)) as u32
        };

        Self {
            pos: vec3(0.0, 0.0, 0.0),
            vel_y: 0.0,
            yaw: -std::f32::consts::FRAC_PI_2,
            pitch: 0.0,
            grounded: true,
            double_jump_used: false,
            crouching: false,
            stamina: STAMINA_MAX,
            health: max_hp,
            max_health: max_hp,
            armor: 0.0,
            bob: 0.0,
            hurt_flash: 0.0,
            shake: 0.0,
            equipped: w,
            ammo: mag,
            reserve: if wdef.is_melee { 0 } else { mag * 6 },
            reload_t: 0.0,
            cooldown: 0.0,
            recoil: 0.0,
            ads: false,
            coins: save.coins,
            ability_lv: save.ability_levels,
            grenade_count: 3,
            grenade_cd: 0.0,
            grenades: Vec::new(),
            pillars,
            floor: def.floor,
            wall: def.wall,
            accent: def.accent,
            map_name: def.name,
            enemies: Vec::new(),
            tracers: Vec::new(),
            particles: Vec::new(),
            shells: Vec::new(),
            floaters: Vec::new(),
            projectiles: Vec::new(),
            wave: 0,
            to_spawn: 0,
            spawn_t: 0.0,
            wave_break: 2.5,
            kills: 0,
            score: 0,
            hit_mark: 0.0,
            muzzle_flash: 0.0,
            recoil_mult,
            spread_mult,
            reload_mult,
            damage_mult,
            mag_bonus,
            zoom_mult,
        }
    }

    fn eye_height(&self) -> f32 {
        if self.crouching {
            EYE_CROUCH
        } else {
            EYE_STAND
        }
    }
    fn eye(&self) -> Vec3 {
        vec3(
            self.pos.x,
            self.eye_height() + self.bob.sin() * 0.035,
            self.pos.z,
        )
    }
    fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let f = look_dir(self.yaw, self.pitch);
        let r = f.cross(Vec3::Y).normalize();
        let u = r.cross(f);
        (f, r, u)
    }
    fn current_mag(&self) -> u32 {
        if WEAPONS[self.equipped].is_melee {
            return 0;
        }
        ((WEAPONS[self.equipped].mag_size as i32 + self.mag_bonus).max(1)) as u32
    }
    fn current_reload(&self) -> f32 {
        if WEAPONS[self.equipped].is_melee {
            return 0.0;
        }
        WEAPONS[self.equipped].reload_time
            * self.reload_mult
            * (1.0 - self.ability_lv[2] as f32 * 0.10).max(0.3)
    }

    fn burst(&mut self, at: Vec3, n: usize, color: Color, speed: f32) {
        for _ in 0..n {
            let v = vec3(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(0.0, 1.5),
                rand::gen_range(-1.0, 1.0),
            ) * speed;
            self.particles.push(Particle {
                pos: at,
                vel: v,
                life: 0.6,
                max_life: 0.6,
                color,
                size: 0.06,
            });
        }
    }

    fn reap_dead(&mut self) {
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].hp <= 0.0 {
                let e = self.enemies.swap_remove(i);
                let reward = e.kind.score();
                self.score += reward;
                self.coins += reward / 2;
                self.kills += 1;
                let center = vec3(e.pos.x, e.kind.height() * 0.4, e.pos.z);
                self.burst(center, 16, e.kind.color(), 5.0);
                // Drop chances
                let roll = rand::gen_range(0.0, 1.0);
                if roll < 0.12 && self.armor < 50.0 {
                    self.armor = (self.armor + 10.0).min(50.0);
                } else if roll < 0.22 {
                    self.reserve += 15;
                } else if roll < 0.25 {
                    self.grenade_count += 1;
                }
                self.floaters.push(FloatingText {
                    pos: vec3(center.x, center.y + 1.0, center.z),
                    text: format!("+{}", reward / 2),
                    life: 1.0,
                    color: GOLD,
                });
            } else {
                i += 1;
            }
        }
    }

    fn shoot(&mut self) {
        let w = WEAPONS[self.equipped];
        self.cooldown = 1.0 / w.fire_rate;
        self.recoil = (self.recoil + w.recoil * 0.12 * self.recoil_mult).min(0.9);
        self.muzzle_flash = 0.06;
        if !w.is_melee {
            self.ammo = self.ammo.saturating_sub(1);
        }

        let (f, r, u) = self.basis();
        let o = self.eye();
        let muzzle = if w.is_melee {
            o + f * 0.5 + r * 0.2 - u * 0.15
        } else {
            o + f * 1.0 + r * 0.25 - u * 0.20
        };

        if !w.is_melee {
            let shell_vel = r * 2.5
                + u * 2.5
                + vec3(rand::gen_range(-0.5, 0.5), 0.0, rand::gen_range(-0.5, 0.5));
            self.shells.push(Shell {
                pos: muzzle,
                vel: shell_vel,
                life: 1.5,
            });
        }

        let total_pellets = w.pellets;
        let spread = w.spread * self.spread_mult * if self.ads { 0.4 } else { 1.0 };
        let dmg_base = w.damage * self.damage_mult * (1.0 + self.ability_lv[5] as f32 * 0.10);

        let mut hits: Vec<(usize, f32, Vec3, bool)> = Vec::new();

        for _ in 0..total_pellets {
            let dir = if spread > 0.0 {
                (f + r * rand::gen_range(-spread, spread) + u * rand::gen_range(-spread, spread))
                    .normalize()
            } else {
                f
            };

            let mut best_t = w.range;
            let mut best_idx: Option<usize> = None;
            let mut headshot = false;

            for (i, e) in self.enemies.iter().enumerate() {
                let h = e.kind.height();
                let rad = (e.kind.radius() * 1.6).min(h * 0.5);
                let c = vec3(e.pos.x, h * 0.5, e.pos.z);
                let oc = c - o;
                let tca = oc.dot(dir);
                if tca < 0.0 {
                    continue;
                }
                let d2 = oc.dot(oc) - tca * tca;
                if d2 > rad * rad {
                    continue;
                }
                let t = tca - (rad * rad - d2).sqrt();
                if t > 0.0 && t < best_t {
                    best_t = t;
                    best_idx = Some(i);
                    let hit_y = o.y + dir.y * t;
                    headshot = hit_y > h * 0.75;
                }
            }

            if !w.is_melee {
                for p in &self.pillars {
                    let ox = o.x - p.x;
                    let oz = o.z - p.z;
                    let a = dir.x * dir.x + dir.z * dir.z;
                    if a < 1e-6 {
                        continue;
                    }
                    let b = 2.0 * (ox * dir.x + oz * dir.z);
                    let c = ox * ox + oz * oz - PILLAR_R * PILLAR_R;
                    let disc = b * b - 4.0 * a * c;
                    if disc >= 0.0 {
                        let t = (-b - disc.sqrt()) / (2.0 * a);
                        if t > 0.0 && t < best_t && o.y + dir.y * t < 4.0 {
                            best_t = t;
                            best_idx = None;
                        }
                    }
                }

                for (oo, dd) in [(o.x, dir.x), (o.z, dir.z)] {
                    if dd.abs() > 1e-5 {
                        let t = if dd > 0.0 {
                            (HALF - oo) / dd
                        } else {
                            (-HALF - oo) / dd
                        };
                        if t > 0.0 && t < best_t {
                            best_t = t;
                            best_idx = None;
                        }
                    }
                }
                if dir.y < -1e-4 {
                    let t = -o.y / dir.y;
                    if t > 0.0 && t < best_t {
                        best_t = t;
                        best_idx = None;
                    }
                }
            }

            let end = o + dir * best_t;
            if !w.is_melee {
                self.tracers.push(Tracer {
                    a: muzzle,
                    b: end,
                    life: 0.07,
                });
                self.burst(end, 2, Color::new(0.9, 0.85, 0.5, 1.0), 3.0);
            }

            if let Some(i) = best_idx {
                let mut dmg = dmg_base;
                if headshot {
                    dmg *= 2.0 + self.ability_lv[8] as f32 * 0.20;
                }
                hits.push((i, dmg, end, headshot));
            } else if w.is_melee {
                self.burst(o + f * 1.5, 4, Color::new(0.9, 0.9, 0.9, 1.0), 2.0);
            }
        }

        for (i, dmg, point, hs) in hits {
            if i >= self.enemies.len() {
                continue;
            }
            self.enemies[i].hp -= dmg;
            self.enemies[i].flash = 0.08;
            self.hit_mark = 0.12;
            let color = if hs {
                Color::new(1.0, 0.3, 0.3, 1.0)
            } else {
                Color::new(1.0, 0.9, 0.4, 1.0)
            };
            self.burst(point, 4, color, 3.5);
            if hs {
                self.floaters.push(FloatingText {
                    pos: vec3(point.x, point.y + 0.4, point.z),
                    text: "HEADSHOT".to_string(),
                    life: 0.9,
                    color,
                });
            }
            if self.ability_lv[9] > 0 && !w.is_melee {
                let radius = 1.2 + self.ability_lv[9] as f32 * 0.5;
                let aoe = dmg * 0.25 * self.ability_lv[9] as f32;
                for j in 0..self.enemies.len() {
                    if j == i {
                        continue;
                    }
                    let d = vec3(
                        self.enemies[j].pos.x - point.x,
                        0.0,
                        self.enemies[j].pos.z - point.z,
                    )
                    .length();
                    if d < radius {
                        self.enemies[j].hp -= aoe;
                    }
                }
                self.burst(point, 6, Color::new(1.0, 0.5, 0.2, 1.0), 4.0);
            }
        }

        self.reap_dead();
    }

    fn throw_grenade(&mut self) {
        if self.grenade_count == 0 || self.grenade_cd > 0.0 {
            return;
        }
        let (f, _, _) = self.basis();
        let eye = self.eye();
        let vel = f * 16.0 + vec3(0.0, 4.0, 0.0);
        self.grenades.push(Grenade {
            pos: eye + f * 0.4,
            vel,
            fuse: 2.5,
        });
        self.grenade_count -= 1;
        self.grenade_cd = 0.4;
    }

    fn update_grenades(&mut self, dt: f32) {
        self.grenade_cd = (self.grenade_cd - dt).max(0.0);
        for g in self.grenades.iter_mut() {
            g.fuse -= dt;
            g.vel.y -= GRAVITY * dt;
            g.pos += g.vel * dt;
            if g.pos.y < 0.15 {
                g.pos.y = 0.15;
                g.vel.y *= -0.3;
                g.vel.x *= 0.7;
                g.vel.z *= 0.7;
            }
            if g.pos.x.abs() > HALF - 0.2 {
                g.pos.x = g.pos.x.clamp(-HALF + 0.2, HALF - 0.2);
                g.vel.x *= -0.5;
            }
            if g.pos.z.abs() > HALF - 0.2 {
                g.pos.z = g.pos.z.clamp(-HALF + 0.2, HALF - 0.2);
                g.vel.z *= -0.5;
            }
        }
        // Detonate expired
        let mut boom: Vec<Vec3> = Vec::new();
        for g in &self.grenades {
            if g.fuse <= 0.0 {
                boom.push(g.pos);
            }
        }
        self.grenades.retain(|g| g.fuse > 0.0);
        for pos in boom {
            let radius = 6.0;
            let dmg_max = 250.0;
            for e in self.enemies.iter_mut() {
                let d = vec3(e.pos.x - pos.x, 0.0, e.pos.z - pos.z).length();
                if d < radius {
                    let fall = 1.0 - (d / radius);
                    e.hp -= dmg_max * fall;
                }
            }
            let pd = vec3(self.pos.x - pos.x, 0.0, self.pos.z - pos.z).length();
            if pd < radius * 0.7 {
                let dmg = 30.0 * (1.0 - pd / (radius * 0.7));
                let absorb = dmg.min(self.armor);
                self.armor -= absorb;
                self.health -= dmg - absorb;
                self.hurt_flash = 0.5;
            }
            self.burst(pos, 50, Color::new(1.0, 0.6, 0.2, 1.0), 10.0);
            self.burst(pos, 20, Color::new(1.0, 0.9, 0.3, 1.0), 6.0);
            self.shake = 1.0;
        }
        self.reap_dead();
    }

    fn switch_weapon(&mut self, idx: usize, save: &Save) {
        if idx >= 9 || !save.owned_weapons[idx] || idx == self.equipped {
            return;
        }
        self.equipped = idx;
        self.reload_t = 0.0;
        self.cooldown = 0.35;
        let (recoil_mult, spread_mult, reload_mult, damage_mult, mag_bonus, zoom_mult) =
            compute_mods(save, idx);
        self.recoil_mult = recoil_mult;
        self.spread_mult = spread_mult;
        self.reload_mult = reload_mult;
        self.damage_mult = damage_mult;
        self.mag_bonus = mag_bonus;
        self.zoom_mult = zoom_mult;
        let w = WEAPONS[idx];
        if w.is_melee {
            self.ammo = 0;
            self.reserve = 0;
        } else {
            self.ammo = 0;
            self.reserve = self.reserve.max(w.mag_size * 4);
        }
    }

    fn spawn_zombie(&mut self, wave: u32) {
        let side = rand::gen_range(0, 4);
        let t = rand::gen_range(-HALF + 1.5, HALF - 1.5);
        let e = HALF - 1.0;
        let (x, z) = match side {
            0 => (t, -e),
            1 => (t, e),
            2 => (-e, t),
            _ => (e, t),
        };
        let roll = rand::gen_range(0.0, 1.0);
        let kind = if wave >= 5 && roll < 0.04 {
            ZombieKind::Boss
        } else if wave >= 3 && roll < 0.12 {
            ZombieKind::Tank
        } else if wave >= 2 && roll < 0.28 {
            ZombieKind::Runner
        } else if wave >= 3 && roll < 0.40 {
            ZombieKind::Spitter
        } else if wave >= 2 && roll < 0.55 {
            ZombieKind::Crawler
        } else {
            ZombieKind::Walker
        };
        let hp = kind.hp(wave);
        self.enemies.push(Enemy {
            pos: vec3(x, 0.0, z),
            kind,
            hp,
            max_hp: hp,
            flash: 0.0,
            atk_cd: 0.6,
            phase: rand::gen_range(0.0, 6.0),
            shoot_cd: 2.0,
            is_boss: kind == ZombieKind::Boss,
        });
    }

    fn update(&mut self, dt: f32, look: Vec2, save: &Save, mouse_wheel_y: f32) {
        // Look FIRST
        self.yaw += look.x * SENS;
        self.pitch -= look.y * SENS;
        if is_key_down(KeyCode::Left) {
            self.yaw -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.yaw += 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            self.pitch += 1.5 * dt;
        }
        if is_key_down(KeyCode::Down) {
            self.pitch -= 1.5 * dt;
        }
        self.pitch = self.pitch.clamp(-1.5, 1.5);

        let fwd = vec3(self.yaw.cos(), 0.0, self.yaw.sin());
        let right = vec3(-self.yaw.sin(), 0.0, self.yaw.cos());

        // Weapon switch (1-9)
        let num = [
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::Key5,
            KeyCode::Key6,
            KeyCode::Key7,
            KeyCode::Key8,
            KeyCode::Key9,
        ];
        for (i, k) in num.iter().enumerate() {
            if is_key_pressed(*k) {
                self.switch_weapon(i, save);
            }
        }
        if mouse_wheel_y.abs() > 0.1 {
            let dir = if mouse_wheel_y > 0.0 { -1 } else { 1 };
            let mut idx = self.equipped as i32;
            for _ in 0..9 {
                idx = (idx + dir).rem_euclid(9);
                if save.owned_weapons[idx as usize] {
                    self.switch_weapon(idx as usize, save);
                    break;
                }
            }
        }

        // Crouch
        self.crouching = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::C);
        let sprinting = is_key_down(KeyCode::LeftShift)
            && !self.crouching
            && self.stamina > 0.0
            && self.grounded;

        let speed = if self.crouching {
            CROUCH_SPEED
        } else if sprinting {
            SPRINT_SPEED * (1.0 + self.ability_lv[6] as f32 * 0.10)
        } else {
            WALK_SPEED * (1.0 + self.ability_lv[0] as f32 * 0.08)
        };

        if sprinting {
            self.stamina = (self.stamina - STAMINA_DRAIN * dt).max(0.0);
        } else {
            self.stamina = (self.stamina + STAMINA_REGEN * dt).min(STAMINA_MAX);
        }

        let mut mv = Vec3::ZERO;
        if is_key_down(KeyCode::W) {
            mv += fwd;
        }
        if is_key_down(KeyCode::S) {
            mv -= fwd;
        }
        if is_key_down(KeyCode::D) {
            mv += right;
        }
        if is_key_down(KeyCode::A) {
            mv -= right;
        }
        if mv.length() > 0.0 {
            mv = mv.normalize() * speed;
            self.bob += dt * 10.0;
        }
        self.pos += mv * dt;
        push_out(&mut self.pos, PLAYER_R, &self.pillars);

        // Jump + double jump (Space ONLY jumps)
        if is_key_pressed(KeyCode::Space) {
            if self.grounded {
                self.vel_y = JUMP_VEL;
                self.grounded = false;
                self.double_jump_used = false;
            } else if self.ability_lv[1] > 0 && !self.double_jump_used {
                self.vel_y = JUMP_VEL * 0.85;
                self.double_jump_used = true;
            }
        }
        self.vel_y -= GRAVITY * dt;
        self.pos.y += self.vel_y * dt;
        if self.pos.y <= 0.0 {
            self.pos.y = 0.0;
            self.vel_y = 0.0;
            self.grounded = true;
            self.double_jump_used = false;
        }

        self.ads = is_mouse_button_down(MouseButton::Right);

        // Tick
        self.cooldown -= dt;
        self.muzzle_flash = (self.muzzle_flash - dt).max(0.0);
        self.recoil = (self.recoil - dt * 4.0).max(0.0);
        self.hit_mark = (self.hit_mark - dt).max(0.0);
        self.hurt_flash = (self.hurt_flash - dt).max(0.0);
        self.shake = (self.shake - dt * 3.0).max(0.0);

        // Grenade throw
        if is_key_pressed(KeyCode::G) {
            self.throw_grenade();
        }

        let w = WEAPONS[self.equipped];
        // FIRE: LMB only (Space no longer shoots)
        let want_fire = if w.auto {
            is_mouse_button_down(MouseButton::Left)
        } else {
            is_mouse_button_pressed(MouseButton::Left)
        };

        // Reload tick
        if self.reload_t > 0.0 {
            self.reload_t -= dt;
            if self.reload_t <= 0.0 {
                self.reload_t = 0.0;
                let need = self.current_mag() - self.ammo;
                let take = need.min(self.reserve);
                self.ammo += take;
                self.reserve -= take;
            }
        }

        // Fire / reload
        if self.reload_t <= 0.0 {
            if w.is_melee {
                if want_fire && self.cooldown <= 0.0 {
                    self.shoot();
                }
            } else {
                if is_key_pressed(KeyCode::R) && self.ammo < self.current_mag() && self.reserve > 0
                {
                    self.reload_t = self.current_reload();
                } else if want_fire && self.cooldown <= 0.0 {
                    if self.ammo > 0 {
                        self.shoot();
                    } else if self.reserve > 0 {
                        self.reload_t = self.current_reload();
                    }
                }
            }
        }

        // Waves
        if self.to_spawn > 0 {
            self.spawn_t -= dt;
            if self.spawn_t <= 0.0 {
                self.spawn_zombie(self.wave);
                self.to_spawn -= 1;
                self.spawn_t = (1.2 - self.wave as f32 * 0.05).max(0.25);
            }
        } else if self.enemies.is_empty() {
            self.wave_break -= dt;
            if self.wave_break <= 0.0 {
                self.wave += 1;
                self.to_spawn = 5 + self.wave * 2;
                self.wave_break = 4.0;
                self.health = (self.health + 30.0).min(self.max_health);
                self.coins += 50 * self.wave;
                self.grenade_count += 1;
            }
        }

        // Zombie AI
        let target = self.pos;
        let mut damage_taken = 0.0;
        let mut spit_requests: Vec<(Vec3, Vec3)> = Vec::new();
        for e in self.enemies.iter_mut() {
            let dx = target.x - e.pos.x;
            let dz = target.z - e.pos.z;
            let dist = (dx * dx + dz * dz).sqrt();
            let stop = e.kind.radius() + PLAYER_R + 0.3;

            if e.kind == ZombieKind::Spitter {
                if dist > 12.0 && dist > 0.01 {
                    let dir = vec3(dx / dist, 0.0, dz / dist);
                    e.pos += dir * e.kind.speed() * dt;
                } else if dist < 7.0 && dist > 0.01 {
                    let dir = vec3(dx / dist, 0.0, dz / dist);
                    e.pos -= dir * e.kind.speed() * 0.7 * dt;
                }
                e.shoot_cd -= dt;
                if e.shoot_cd <= 0.0 && dist < 15.0 {
                    let from = vec3(e.pos.x, e.kind.height() * 0.7, e.pos.z);
                    let to = vec3(target.x, EYE_STAND * 0.5, target.z);
                    spit_requests.push((from, to));
                    e.shoot_cd = 2.5;
                }
            } else {
                let speed_mult = if e.is_boss && e.hp < e.max_hp * 0.5 {
                    1.6
                } else {
                    1.0
                };
                if dist > stop && dist > 0.01 {
                    let dir = vec3(dx / dist, 0.0, dz / dist);
                    e.pos += dir * e.kind.speed() * speed_mult * dt;
                }
                if dist < stop + 0.4 {
                    e.atk_cd -= dt;
                    if e.atk_cd <= 0.0 {
                        let m = if e.is_boss && e.hp < e.max_hp * 0.5 {
                            1.5
                        } else {
                            1.0
                        };
                        damage_taken += e.kind.damage() * m;
                        e.atk_cd = 0.8;
                    }
                }
            }
            e.phase += dt * e.kind.speed() * 2.0;
            e.flash = (e.flash - dt).max(0.0);
            push_out(&mut e.pos, e.kind.radius(), &self.pillars);
        }

        for (from, to) in spit_requests {
            let dir = (to - from).normalize();
            self.projectiles.push(Projectile {
                pos: from,
                vel: dir * 14.0,
                life: 2.0,
                damage: 12.0,
            });
        }

        // Separation
        let n = self.enemies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (a, b) = self.enemies.split_at_mut(j);
                let ea = &mut a[i];
                let eb = &mut b[0];
                let d = vec3(ea.pos.x - eb.pos.x, 0.0, ea.pos.z - eb.pos.z);
                let len = d.length();
                let min = ea.kind.radius() + eb.kind.radius();
                if len < min && len > 0.001 {
                    let push = d / len * (min - len) * 0.5;
                    ea.pos += push;
                    eb.pos -= push;
                }
            }
        }

        // Projectiles
        for p in self.projectiles.iter_mut() {
            p.life -= dt;
            p.pos += p.vel * dt;
            let d = vec3(p.pos.x - target.x, 0.0, p.pos.z - target.z).length();
            if d < PLAYER_R + 0.3 && p.life > 0.0 {
                damage_taken += p.damage;
                p.life = 0.0;
            }
            if p.pos.y < 0.1 {
                p.life = 0.0;
            }
        }
        self.projectiles.retain(|p| p.life > 0.0);

        // Grenades
        self.update_grenades(dt);

        // Apply damage
        if damage_taken > 0.0 {
            let absorb = damage_taken.min(self.armor);
            self.armor -= absorb;
            self.health -= damage_taken - absorb;
            self.hurt_flash = 0.35;
            self.shake = (self.shake + 0.4).min(1.0);
        }

        // Regen
        if self.ability_lv[4] > 0 && self.health > 0.0 {
            let rate = self.ability_lv[4] as f32;
            self.health = (self.health + rate * dt).min(self.max_health);
        }

        // Effects
        for t in self.tracers.iter_mut() {
            t.life -= dt;
        }
        self.tracers.retain(|t| t.life > 0.0);
        for p in self.particles.iter_mut() {
            p.life -= dt;
            p.vel.y -= 15.0 * dt;
            p.pos += p.vel * dt;
            if p.pos.y < 0.05 {
                p.pos.y = 0.05;
                p.vel.y *= -0.3;
                p.vel.x *= 0.7;
                p.vel.z *= 0.7;
            }
        }
        self.particles.retain(|p| p.life > 0.0);
        for s in self.shells.iter_mut() {
            s.life -= dt;
            s.vel.y -= 15.0 * dt;
            s.pos += s.vel * dt;
            if s.pos.y < 0.05 {
                s.pos.y = 0.05;
                s.vel.y *= -0.4;
                s.vel.x *= 0.6;
                s.vel.z *= 0.6;
            }
        }
        self.shells.retain(|s| s.life > 0.0);
        for f in self.floaters.iter_mut() {
            f.life -= dt;
            f.pos.y += dt * 1.5;
        }
        self.floaters.retain(|f| f.life > 0.0);
    }

    fn draw_world(&self) {
        let (f, r, u) = self.basis();
        let eye = self.eye();
        let fov = if self.ads {
            75.0f32.to_radians() / self.zoom_mult.max(1.0)
        } else {
            75.0f32.to_radians()
        };
        let shake_off = vec3(
            rand::gen_range(-1.0, 1.0) * self.shake * 0.05,
            rand::gen_range(-1.0, 1.0) * self.shake * 0.05,
            0.0,
        );
        set_camera(&Camera3D {
            position: eye + shake_off,
            target: eye + shake_off + f,
            up: vec3(0.0, 1.0, 0.0),
            fovy: fov,
            ..Default::default()
        });

        draw_plane(vec3(0.0, -0.01, 0.0), vec2(HALF, HALF), None, self.floor);
        draw_grid(
            48,
            1.0,
            Color::new(0.24, 0.28, 0.40, 1.0),
            Color::new(0.16, 0.18, 0.26, 1.0),
        );

        let len = HALF * 2.0 + 1.0;
        draw_cube(
            vec3(0.0, 2.0, -HALF - 0.5),
            vec3(len, 4.0, 1.0),
            None,
            self.wall,
        );
        draw_cube(
            vec3(0.0, 2.0, HALF + 0.5),
            vec3(len, 4.0, 1.0),
            None,
            self.wall,
        );
        draw_cube(
            vec3(-HALF - 0.5, 2.0, 0.0),
            vec3(1.0, 4.0, len),
            None,
            self.wall,
        );
        draw_cube(
            vec3(HALF + 0.5, 2.0, 0.0),
            vec3(1.0, 4.0, len),
            None,
            self.wall,
        );

        for p in &self.pillars {
            let c = vec3(p.x, 2.0, p.z);
            draw_cube(c, vec3(1.6, 4.0, 1.6), None, self.wall);
            draw_cube_wires(c, vec3(1.6, 4.0, 1.6), self.accent);
        }

        for e in &self.enemies {
            let h = e.kind.height();
            let w = e.kind.radius() * 1.6;
            let bob = e.phase.sin().abs() * 0.10;
            let col = if e.flash > 0.0 { WHITE } else { e.kind.color() };
            let body_y = if e.kind == ZombieKind::Crawler {
                h * 0.3
            } else {
                h * 0.35
            };
            let body_h = if e.kind == ZombieKind::Crawler {
                h * 0.5
            } else {
                h * 0.7
            };
            let body = vec3(e.pos.x, body_y + bob, e.pos.z);
            draw_cube(body, vec3(w, body_h, w * 0.8), None, col);
            draw_cube_wires(body, vec3(w, body_h, w * 0.8), BLACK);

            if e.kind != ZombieKind::Crawler {
                let head = vec3(e.pos.x, h * 0.85 + bob, e.pos.z);
                draw_sphere(head, w * 0.35, None, col);
                if e.is_boss {
                    draw_sphere(head, w * 0.5, None, Color::new(1.0, 0.2, 0.2, 0.25));
                }
            }

            if e.hp < e.max_hp {
                let hp_pct = (e.hp / e.max_hp).clamp(0.0, 1.0);
                let bar_w = w * 1.8;
                let bar_y = h + 0.3;
                draw_cube(
                    vec3(e.pos.x, bar_y, e.pos.z),
                    vec3(bar_w, 0.06, 0.03),
                    None,
                    Color::new(0.1, 0.1, 0.1, 0.8),
                );
                draw_cube(
                    vec3(e.pos.x - bar_w * (1.0 - hp_pct) * 0.5, bar_y, e.pos.z),
                    vec3(bar_w * hp_pct, 0.06, 0.04),
                    None,
                    Color::new(0.9, 0.2, 0.2, 0.95),
                );
            }
        }

        for p in &self.projectiles {
            draw_sphere(p.pos, 0.2, None, Color::new(0.3, 1.0, 0.3, 1.0));
        }
        for g in &self.grenades {
            draw_sphere(g.pos, 0.15, None, Color::new(0.3, 0.7, 0.3, 1.0));
            if g.fuse < 0.5 {
                draw_sphere(g.pos, 0.25, None, Color::new(1.0, 0.4, 0.2, 0.4));
            }
        }
        for p in &self.particles {
            let a = (p.life / p.max_life).min(1.0);
            let c = Color::new(p.color.r, p.color.g, p.color.b, a);
            draw_cube(p.pos, vec3(p.size, p.size, p.size), None, c);
        }
        for s in &self.shells {
            draw_cube(
                s.pos,
                vec3(0.03, 0.03, 0.05),
                None,
                Color::new(0.9, 0.7, 0.3, 1.0),
            );
        }
        for t in &self.tracers {
            draw_line_3d(t.a, t.b, Color::new(1.0, 0.9, 0.4, 1.0));
        }

        self.draw_weapon(eye, f, r, u);
    }

    fn draw_weapon(&self, eye: Vec3, f: Vec3, r: Vec3, u: Vec3) {
        let w = WEAPONS[self.equipped];
        let reload_offset = if self.reload_t > 0.0 {
            let p = 1.0 - (self.reload_t / self.current_reload().max(0.01));
            (p * std::f32::consts::PI).sin() * 0.3
        } else {
            0.0
        };
        let ads_offset = if self.ads { 0.1 } else { 0.0 };
        let base = eye
            + f * (0.55 + ads_offset - self.recoil * 0.08 - reload_offset * 0.5)
            + r * (if self.ads { 0.05 } else { 0.30 })
            - u * (0.26 - reload_offset * 0.4);

        let seg_len = 0.11;
        if !w.is_melee {
            for k in 0..w.model_len {
                let t = k as f32 * seg_len;
                let c = if k == w.model_len - 1 {
                    w.color
                } else {
                    DARKGRAY
                };
                draw_sphere(base + f * t, w.model_radius, None, c);
            }
        }

        // Per-weapon silhouette
        match self.equipped {
            0 => {
                draw_cube(base - u * 0.10, vec3(0.09, 0.18, 0.09), None, DARKGRAY);
            }
            1 => {
                // KNIFE - small blade
                draw_cube(
                    base - u * 0.05,
                    vec3(0.04, 0.14, 0.03),
                    None,
                    Color::new(0.25, 0.25, 0.30, 1.0),
                );
                draw_cube(
                    base + f * 0.18,
                    vec3(0.03, 0.10, 0.02),
                    None,
                    Color::new(0.9, 0.9, 0.95, 1.0),
                );
            }
            2 => {
                draw_cube(
                    base - u * 0.12 - f * 0.04,
                    vec3(0.07, 0.22, 0.10),
                    None,
                    DARKGRAY,
                );
            }
            3 => {
                draw_cube(base - u * 0.04, vec3(0.17, 0.10, 0.20), None, DARKBROWN);
                draw_sphere(base + f * 0.72, 0.09, None, w.color);
                draw_sphere(base + f * 0.72 + u * 0.10, 0.09, None, w.color);
            }
            4 => {
                draw_cube(base - u * 0.13, vec3(0.07, 0.20, 0.09), None, DARKGREEN);
                draw_sphere(base + f * 0.35 + u * 0.10, 0.05, None, w.color);
            }
            5 => {
                draw_sphere(
                    base + f * 0.4 + u * 0.12,
                    0.09,
                    None,
                    Color::new(0.2, 0.2, 0.25, 1.0),
                );
                draw_cube(base - u * 0.14, vec3(0.07, 0.18, 0.10), None, DARKPURPLE);
            }
            6 => {
                draw_cube(base - u * 0.13, vec3(0.10, 0.22, 0.10), None, DARKGRAY);
                draw_cube(
                    base + f * 0.3 + r * 0.12,
                    vec3(0.15, 0.15, 0.25),
                    None,
                    Color::new(0.4, 0.4, 0.4, 1.0),
                );
            }
            7 => {
                // REVOLVER - chunky grip + cylinder
                draw_cube(
                    base - u * 0.11,
                    vec3(0.10, 0.20, 0.10),
                    None,
                    Color::new(0.35, 0.20, 0.10, 1.0),
                );
                draw_sphere(base + f * 0.1 + u * 0.02, 0.08, None, w.color);
            }
            8 => {
                // DMR - rifle with small scope
                draw_sphere(
                    base + f * 0.3 + u * 0.12,
                    0.06,
                    None,
                    Color::new(0.25, 0.25, 0.30, 1.0),
                );
                draw_cube(
                    base - u * 0.13,
                    vec3(0.07, 0.20, 0.09),
                    None,
                    Color::new(0.15, 0.35, 0.25, 1.0),
                );
            }
            _ => {}
        }

        if self.muzzle_flash > 0.0 && !w.is_melee {
            let a = self.muzzle_flash / 0.06;
            let flash_pos = base + f * (w.model_len as f32 * seg_len + 0.1);
            draw_sphere(flash_pos, 0.14, None, Color::new(1.0, 0.8, 0.2, a));
            draw_sphere(flash_pos, 0.25, None, Color::new(1.0, 0.6, 0.1, a * 0.4));
        }
    }

    fn draw_hud(&self) {
        let sw = screen_width();
        let sh = screen_height();

        if self.hurt_flash > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                sw,
                sh,
                Color::new(1.0, 0.0, 0.0, self.hurt_flash * 0.5),
            );
        }

        if !self.ads {
            let (cx, cy) = (sw / 2.0, sh / 2.0);
            let gap = 3.0 + WEAPONS[self.equipped].spread * self.spread_mult * 200.0;
            let len = 8.0;
            let col = Color::new(1.0, 1.0, 1.0, 0.9);
            draw_line(cx - gap - len, cy, cx - gap, cy, 2.0, col);
            draw_line(cx + gap, cy, cx + gap + len, cy, 2.0, col);
            draw_line(cx, cy - gap - len, cx, cy - gap, 2.0, col);
            draw_line(cx, cy + gap, cx, cy + gap + len, 2.0, col);
            draw_circle(cx, cy, 1.5, col);
        } else {
            let (cx, cy) = (sw / 2.0, sh / 2.0);
            draw_circle_lines(cx, cy, 3.0, 1.0, Color::new(1.0, 1.0, 1.0, 0.9));
            draw_circle_lines(cx, cy, 60.0, 1.5, Color::new(1.0, 1.0, 1.0, 0.3));
        }

        if self.hit_mark > 0.0 {
            let a = (self.hit_mark / 0.12).clamp(0.0, 1.0);
            let c = Color::new(1.0, 0.3, 0.3, a);
            let (cx, cy) = (sw / 2.0, sh / 2.0);
            let d = 6.0;
            let e = 13.0;
            draw_line(cx - e, cy - e, cx - d, cy - d, 2.5, c);
            draw_line(cx + d, cy + d, cx + e, cy + e, 2.5, c);
            draw_line(cx + e, cy - e, cx + d, cy - d, 2.5, c);
            draw_line(cx - d, cy + d, cx - e, cy + e, 2.5, c);
        }

        // Bottom-left panel
        let px = 20.0;
        let py = sh - 130.0;
        let pw = 300.0;
        ui_panel(px, py, pw, 110.0, false);
        ui_text(
            "HEALTH",
            px + 12.0,
            py + 22.0,
            16.0,
            Color::new(0.6, 0.7, 0.8, 1.0),
        );
        let hp_pct = (self.health / self.max_health).max(0.0);
        ui_bar(
            px + 12.0,
            py + 28.0,
            pw - 24.0,
            14.0,
            hp_pct,
            Color::new(0.2, 0.9, 0.35, 1.0),
        );
        ui_text(
            "ARMOR",
            px + 12.0,
            py + 60.0,
            16.0,
            Color::new(0.6, 0.7, 0.8, 1.0),
        );
        ui_bar(
            px + 12.0,
            py + 66.0,
            pw - 24.0,
            10.0,
            (self.armor / 50.0).min(1.0),
            Color::new(0.4, 0.65, 1.0, 1.0),
        );
        ui_text(
            "STAMINA",
            px + 12.0,
            py + 94.0,
            14.0,
            Color::new(0.6, 0.7, 0.8, 1.0),
        );
        ui_bar(
            px + 90.0,
            py + 84.0,
            pw - 102.0,
            8.0,
            self.stamina / STAMINA_MAX,
            Color::new(1.0, 0.85, 0.2, 1.0),
        );

        // Bottom-right panel
        let ww = 280.0;
        let wx = sw - ww - 20.0;
        let wy = sh - 130.0;
        ui_panel(wx, wy, ww, 110.0, false);
        let w = WEAPONS[self.equipped];
        ui_text(w.name, wx + 12.0, wy + 24.0, 22.0, w.color);
        ui_text(
            w.rarity.name(),
            wx + 12.0,
            wy + 44.0,
            12.0,
            w.rarity.color(),
        );
        let ammo_str = if w.is_melee {
            "MELEE".to_string()
        } else {
            format!("{}/{}", self.ammo, self.reserve)
        };
        let d = measure_text(&ammo_str, None, 40, 1.0);
        draw_text(&ammo_str, wx + ww - 12.0 - d.width, wy + 72.0, 40.0, WHITE);
        if self.reload_t > 0.0 {
            let p = 1.0 - (self.reload_t / self.current_reload().max(0.01)).clamp(0.0, 1.0);
            ui_bar(
                wx + 12.0,
                wy + 90.0,
                ww - 24.0,
                8.0,
                p,
                Color::new(1.0, 0.7, 0.2, 1.0),
            );
        } else if !w.is_melee {
            ui_text(
                "R to reload  |  G grenade",
                wx + 12.0,
                wy + 98.0,
                14.0,
                Color::new(0.5, 0.55, 0.65, 1.0),
            );
        } else {
            ui_text(
                "G grenade",
                wx + 12.0,
                wy + 98.0,
                14.0,
                Color::new(0.5, 0.55, 0.65, 1.0),
            );
        }

        // Grenade count badge above weapon panel
        let gcolor = if self.grenade_count > 0 {
            Color::new(0.5, 0.9, 0.5, 1.0)
        } else {
            Color::new(0.4, 0.4, 0.4, 1.0)
        };
        draw_text(
            &format!("GRENADES [G]  x{}", self.grenade_count),
            wx + 12.0,
            wy - 10.0,
            18.0,
            gcolor,
        );

        // Top-center
        let wave_str = format!(
            "WAVE {}   |   ZOMBIES {}",
            self.wave,
            self.enemies.len() + self.to_spawn as usize
        );
        let d = measure_text(&wave_str, None, 26, 1.0);
        draw_text(
            &wave_str,
            sw / 2.0 - d.width / 2.0,
            34.0,
            26.0,
            Color::new(0.9, 0.9, 0.9, 1.0),
        );

        // Top-left
        draw_text(&format!("COINS {}", self.coins), 20.0, 30.0, 22.0, GOLD);
        draw_text(
            &format!("KILLS {}", self.kills),
            20.0,
            54.0,
            18.0,
            Color::new(0.7, 0.75, 0.85, 1.0),
        );

        // Top-right
        let map_str = format!("MAP: {}", self.map_name);
        let d = measure_text(&map_str, None, 20, 1.0);
        draw_text(&map_str, sw - 20.0 - d.width, 30.0, 20.0, self.accent);

        if self.to_spawn == 0 && self.enemies.is_empty() {
            let m = format!("WAVE {} INCOMING", self.wave + 1);
            let d = measure_text(&m, None, 46, 1.0);
            draw_text(
                &m,
                sw / 2.0 - d.width / 2.0,
                sh / 2.0 - 100.0,
                46.0,
                Color::new(1.0, 0.85, 0.3, 1.0),
            );
        }
    }
}

// ===================== UI PRIMITIVES =====================
const PANEL_BG: Color = Color::new(0.08, 0.09, 0.13, 0.92);
const PANEL_BG_SEL: Color = Color::new(0.14, 0.17, 0.24, 0.95);
const PANEL_BORDER: Color = Color::new(0.28, 0.32, 0.42, 1.0);

fn ui_panel(x: f32, y: f32, w: f32, h: f32, selected: bool) {
    draw_rectangle(x, y, w, h, if selected { PANEL_BG_SEL } else { PANEL_BG });
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        if selected { 2.0 } else { 1.0 },
        if selected { GOLD } else { PANEL_BORDER },
    );
}
fn ui_bar(x: f32, y: f32, w: f32, h: f32, pct: f32, color: Color) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.0, 0.0, 0.6));
    draw_rectangle(x, y, w * pct.clamp(0.0, 1.0), h, color);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(1.0, 1.0, 1.0, 0.3));
}
fn ui_text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(s, x, y, size, color);
}
fn center_text(s: &str, y: f32, size: f32, color: Color) {
    let d = measure_text(s, None, size as u16, 1.0);
    draw_text(s, screen_width() / 2.0 - d.width / 2.0, y, size, color);
}

fn ui_button(x: f32, y: f32, w: f32, h: f32, label: &str, focused: bool) -> bool {
    let mx = mouse_position().0;
    let my = mouse_position().1;
    let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let sel = focused || hovered;
    ui_panel(x, y, w, h, sel);
    let d = measure_text(label, None, 26, 1.0);
    draw_text(
        label,
        x + w / 2.0 - d.width / 2.0,
        y + h / 2.0 + d.height / 2.0 - 2.0,
        26.0,
        if sel {
            Color::new(1.0, 1.0, 1.0, 1.0)
        } else {
            Color::new(0.75, 0.78, 0.85, 1.0)
        },
    );
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

// ===================== SCREENS =====================
fn draw_menu(save: &Save, sel: &mut usize, map_idx: &mut usize) -> Option<MenuCmd> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.02, 0.03, 0.05, 0.85));

    center_text(
        "DEADZONE",
        sh * 0.15,
        82.0,
        Color::new(1.0, 0.35, 0.25, 1.0),
    );
    center_text(
        "ZOMBIE SURVIVAL",
        sh * 0.15 + 50.0,
        30.0,
        Color::new(0.7, 0.75, 0.85, 1.0),
    );

    draw_text(&format!("COINS {}", save.coins), 30.0, 40.0, 24.0, GOLD);
    draw_text(
        &format!("BEST WAVE {}", save.best_wave),
        30.0,
        70.0,
        20.0,
        Color::new(0.7, 0.75, 0.85, 1.0),
    );
    draw_text(
        &format!("KILLS {}", save.total_kills),
        30.0,
        96.0,
        20.0,
        Color::new(0.7, 0.75, 0.85, 1.0),
    );

    let bw = 340.0;
    let bh = 56.0;
    let bx = sw / 2.0 - bw / 2.0;
    let start_y = sh * 0.42;

    if is_key_pressed(KeyCode::Down) {
        *sel = (*sel + 1) % 6;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel = (*sel + 5) % 6;
    }

    let maps_list = maps();
    let map_name = maps_list[*map_idx % maps_list.len()].name;
    let buttons = [
        "PLAY".to_string(),
        "ARMORY".to_string(),
        "INVENTORY".to_string(),
        "ABILITIES".to_string(),
        format!("MAP: {}", map_name),
        "QUIT".to_string(),
    ];

    let mut cmd: Option<MenuCmd> = None;
    for (i, label) in buttons.iter().enumerate() {
        let y = start_y + i as f32 * (bh + 12.0);
        if ui_button(bx, y, bw, bh, label, *sel == i) {
            *sel = i;
            cmd = Some(match i {
                0 => MenuCmd::Play,
                1 => MenuCmd::Shop,
                2 => MenuCmd::Inventory,
                3 => MenuCmd::Abilities,
                4 => MenuCmd::CycleMap,
                _ => MenuCmd::Quit,
            });
        }
    }

    if cmd.is_none() && is_key_pressed(KeyCode::Enter) {
        cmd = Some(match *sel {
            0 => MenuCmd::Play,
            1 => MenuCmd::Shop,
            2 => MenuCmd::Inventory,
            3 => MenuCmd::Abilities,
            4 => MenuCmd::CycleMap,
            _ => MenuCmd::Quit,
        });
    }
    if cmd.is_none() && is_key_pressed(KeyCode::M) {
        cmd = Some(MenuCmd::CycleMap);
    }
    if cmd.is_none() && is_key_pressed(KeyCode::Q) {
        cmd = Some(MenuCmd::Quit);
    }

    center_text(
        "WASD move | SHIFT sprint | CTRL crouch | SPACE jump",
        sh - 80.0,
        18.0,
        Color::new(0.55, 0.6, 0.7, 1.0),
    );
    center_text(
        "LMB fire | RMB aim | R reload | G grenade | 1-9 weapon",
        sh - 56.0,
        18.0,
        Color::new(0.55, 0.6, 0.7, 1.0),
    );
    center_text(
        "W/S navigate | ENTER select | M cycle map | Q quit",
        sh - 30.0,
        16.0,
        Color::new(0.45, 0.5, 0.6, 1.0),
    );

    cmd
}

fn draw_shop(save: &mut Save, sel: &mut usize) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.03, 0.04, 0.06, 0.95));

    draw_text("ARMORY", 40.0, 56.0, 44.0, Color::new(1.0, 0.6, 0.3, 1.0));
    draw_text(
        &format!("COINS {}", save.coins),
        sw - 220.0,
        52.0,
        26.0,
        GOLD,
    );
    draw_text(
        "CLICK card or ENTER to buy/equip  |  UP/DOWN select  |  ESC back",
        40.0,
        84.0,
        16.0,
        Color::new(0.5, 0.55, 0.65, 1.0),
    );

    if is_key_pressed(KeyCode::Down) {
        *sel = (*sel + 1) % 9;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel = (*sel + 8) % 9;
    }

    let cols = 3;
    let pad_x = 40.0;
    let pad_y = 110.0;
    let gap = 20.0;
    let card_w = (sw - pad_x * 2.0 - gap * (cols - 1) as f32) / cols as f32;
    let card_h = 110.0;

    let mx = mouse_position().0;
    let my = mouse_position().1;
    let mut clicked: Option<usize> = None;

    for i in 0..9 {
        let col = i % cols;
        let row = i / cols;
        let x = pad_x + col as f32 * (card_w + gap);
        let y = pad_y + row as f32 * (card_h + 12.0);
        let w = &WEAPONS[i];
        let owned = save.owned_weapons[i];
        let equipped = save.equipped_weapon == i;
        let hovered = mx >= x && mx <= x + card_w && my >= y && my <= y + card_h;
        let selected = *sel == i || hovered;

        ui_panel(x, y, card_w, card_h, selected);

        draw_text(w.name, x + 16.0, y + 30.0, 24.0, w.color);
        draw_text(w.rarity.name(), x + 16.0, y + 48.0, 12.0, w.rarity.color());

        let sy = y + 66.0;
        if w.is_melee {
            draw_text(
                &format!("DMG {:.0}", w.damage),
                x + 16.0,
                sy,
                13.0,
                Color::new(0.75, 0.8, 0.9, 1.0),
            );
            draw_text("MELEE", x + 110.0, sy, 13.0, Color::new(0.9, 0.8, 0.5, 1.0));
            draw_text(
                &format!("RATE {:.1}", w.fire_rate),
                x + 200.0,
                sy,
                13.0,
                Color::new(0.75, 0.8, 0.9, 1.0),
            );
        } else {
            draw_text(
                &format!("DMG {:.0}", w.damage),
                x + 16.0,
                sy,
                13.0,
                Color::new(0.75, 0.8, 0.9, 1.0),
            );
            draw_text(
                &format!("RATE {:.1}", w.fire_rate),
                x + 100.0,
                sy,
                13.0,
                Color::new(0.75, 0.8, 0.9, 1.0),
            );
            draw_text(
                &format!("MAG {}", w.mag_size),
                x + 185.0,
                sy,
                13.0,
                Color::new(0.75, 0.8, 0.9, 1.0),
            );
            draw_text(
                &format!("RNG {:.0}", w.range),
                x + 260.0,
                sy,
                13.0,
                Color::new(0.75, 0.8, 0.9, 1.0),
            );
        }

        // Status
        let (status, scol) = if equipped {
            ("EQUIPPED".to_string(), Color::new(0.4, 1.0, 0.4, 1.0))
        } else if i == 1 {
            (
                "ALWAYS OWNED - 2".to_string(),
                Color::new(0.6, 0.8, 0.6, 1.0),
            )
        } else if owned {
            (
                "OWNED - CLICK to equip".to_string(),
                Color::new(0.5, 0.8, 1.0, 1.0),
            )
        } else if save.coins >= w.price {
            (format!("BUY {}c", w.price), GOLD)
        } else {
            (format!("{}c", w.price), Color::new(0.5, 0.5, 0.5, 1.0))
        };
        draw_text(
            &status,
            x + card_w - 12.0 - measure_text(&status, None, 14, 1.0).width,
            y + 32.0,
            14.0,
            scol,
        );

        draw_text(
            if w.auto { "AUTO" } else { "SEMI" },
            x + 16.0,
            y + card_h - 14.0,
            11.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );

        // Click detection
        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            clicked = Some(i);
        }
    }

    if let Some(i) = clicked {
        *sel = i;
        let w = &WEAPONS[i];
        if save.owned_weapons[i] {
            save.equipped_weapon = i;
        } else if save.coins >= w.price {
            save.coins -= w.price;
            save.owned_weapons[i] = true;
            save.equipped_weapon = i;
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        let w = &WEAPONS[*sel];
        if save.owned_weapons[*sel] {
            save.equipped_weapon = *sel;
        } else if save.coins >= w.price {
            save.coins -= w.price;
            save.owned_weapons[*sel] = true;
            save.equipped_weapon = *sel;
        }
    }
    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Menu);
    }
    let _ = sh;
    None
}

fn draw_inventory(
    save: &mut Save,
    sel_w: &mut usize,
    sel_slot: &mut usize,
    sel_att: &mut usize,
) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.03, 0.04, 0.06, 0.95));

    draw_text(
        "INVENTORY",
        40.0,
        56.0,
        44.0,
        Color::new(0.6, 0.9, 0.6, 1.0),
    );
    draw_text(
        "LEFT/RIGHT weapon | UP/DOWN slot | 1-9 select attachment | ENTER toggle | ESC back",
        40.0,
        84.0,
        14.0,
        Color::new(0.5, 0.55, 0.65, 1.0),
    );
    draw_text(
        &format!("COINS {}", save.coins),
        sw - 200.0,
        52.0,
        24.0,
        GOLD,
    );

    // Weapon selector row (9 items, 2 rows of 5 to fit)
    let wx = 40.0;
    let wy = 120.0;
    let ww = 115.0;
    for i in 0..9 {
        let col = i % 5;
        let row = i / 5;
        let x = wx + col as f32 * (ww + 6.0);
        let y = wy + row as f32 * (56.0 + 6.0);
        let owned = save.owned_weapons[i];
        ui_panel(x, y, ww, 56.0, *sel_w == i);
        let c = if owned {
            WEAPONS[i].color
        } else {
            Color::new(0.3, 0.3, 0.3, 1.0)
        };
        draw_text(WEAPONS[i].name, x + 6.0, y + 22.0, 13.0, c);
        draw_text(
            if owned { "OWNED" } else { "LOCKED" },
            x + 6.0,
            y + 42.0,
            11.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
    }

    // Attachment slots
    let sx = 40.0;
    let sy = 270.0;
    for slot in 0..5 {
        let y = sy + slot as f32 * 84.0;
        ui_panel(sx, y, 400.0, 78.0, *sel_slot == slot);
        draw_text(
            SLOT_NAMES[slot],
            sx + 12.0,
            y + 22.0,
            16.0,
            Color::new(0.7, 0.8, 0.9, 1.0),
        );
        let cur = save.weapon_attachments[*sel_w][slot];
        if cur >= 0 && (cur as usize) < ATTACHMENTS.len() {
            let a = &ATTACHMENTS[cur as usize];
            draw_text(a.name, sx + 12.0, y + 46.0, 20.0, WEAPONS[*sel_w].color);
            draw_text(
                a.desc,
                sx + 12.0,
                y + 66.0,
                13.0,
                Color::new(0.6, 0.65, 0.75, 1.0),
            );
        } else {
            draw_text(
                "EMPTY",
                sx + 12.0,
                y + 46.0,
                20.0,
                Color::new(0.4, 0.4, 0.4, 1.0),
            );
        }
    }

    // Attachment list
    let ax = 480.0;
    let ay = 270.0;
    let aw = sw - 500.0;
    ui_panel(ax, ay - 8.0, aw, 40.0, false);
    draw_text(
        &format!("AVAILABLE - {}", SLOT_NAMES[*sel_slot]),
        ax + 12.0,
        ay + 18.0,
        18.0,
        Color::new(0.7, 0.8, 0.9, 1.0),
    );

    let mut idx_in_slot = 0;
    let mut slot_att_indices: Vec<usize> = Vec::new();
    for (ai, a) in ATTACHMENTS.iter().enumerate() {
        if a.slot != *sel_slot {
            continue;
        }
        slot_att_indices.push(ai);
        let y = ay + 44.0 + idx_in_slot as f32 * 56.0;
        let owned = save.owned_attachments[ai];
        let is_sel = *sel_att == ai;
        ui_panel(ax, y, aw, 50.0, is_sel);
        draw_text(
            a.name,
            ax + 12.0,
            y + 22.0,
            18.0,
            if owned {
                Color::new(0.8, 0.9, 1.0, 1.0)
            } else {
                Color::new(0.5, 0.5, 0.5, 1.0)
            },
        );
        draw_text(
            a.desc,
            ax + 12.0,
            y + 42.0,
            13.0,
            Color::new(0.6, 0.65, 0.75, 1.0),
        );
        if !owned {
            draw_text(
                &format!("{}c", a.price),
                ax + aw - 80.0,
                y + 32.0,
                15.0,
                GOLD,
            );
        } else if save.weapon_attachments[*sel_w][*sel_slot] == ai as i32 {
            draw_text(
                "ATTACHED",
                ax + aw - 110.0,
                y + 32.0,
                14.0,
                Color::new(0.4, 1.0, 0.4, 1.0),
            );
        }
        idx_in_slot += 1;
    }

    if is_key_pressed(KeyCode::Left) {
        *sel_w = (*sel_w + 8) % 9;
    }
    if is_key_pressed(KeyCode::Right) {
        *sel_w = (*sel_w + 1) % 9;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel_slot = (*sel_slot + 4) % 5;
    }
    if is_key_pressed(KeyCode::Down) {
        *sel_slot = (*sel_slot + 1) % 5;
    }

    let keys = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ];
    for (k_idx, k) in keys.iter().enumerate() {
        if is_key_pressed(*k) {
            if k_idx < slot_att_indices.len() {
                *sel_att = slot_att_indices[k_idx];
            }
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        if save.owned_attachments[*sel_att] {
            if save.weapon_attachments[*sel_w][*sel_slot] == *sel_att as i32 {
                save.weapon_attachments[*sel_w][*sel_slot] = -1;
            } else {
                save.weapon_attachments[*sel_w][*sel_slot] = *sel_att as i32;
            }
        }
    }
    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Menu);
    }
    let _ = sh;
    None
}

fn draw_abilities(save: &mut Save, sel: &mut usize) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.03, 0.04, 0.06, 0.95));

    draw_text(
        "ABILITIES",
        40.0,
        56.0,
        44.0,
        Color::new(0.7, 0.6, 1.0, 1.0),
    );
    draw_text(
        &format!("COINS {}", save.coins),
        sw - 200.0,
        52.0,
        26.0,
        GOLD,
    );
    draw_text(
        "UP/DOWN select  |  ENTER purchase  |  ESC back",
        40.0,
        84.0,
        16.0,
        Color::new(0.5, 0.55, 0.65, 1.0),
    );

    if is_key_pressed(KeyCode::Down) {
        *sel = (*sel + 1) % 10;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel = (*sel + 9) % 10;
    }

    let cw = sw - 80.0;
    let ch = 72.0;
    for i in 0..10 {
        let a = &ABILITIES[i];
        let y = 110.0 + i as f32 * (ch + 8.0);
        let lv = save.ability_levels[i];
        let maxed = lv >= a.max_level;
        let cost = if maxed { 0 } else { ability_cost(i, lv) };
        let affordable = save.coins >= cost;

        ui_panel(40.0, y, cw, ch, *sel == i);
        draw_text(a.name, 56.0, y + 28.0, 24.0, Color::new(0.9, 0.9, 1.0, 1.0));
        draw_text(
            a.desc,
            56.0,
            y + 52.0,
            15.0,
            Color::new(0.6, 0.65, 0.75, 1.0),
        );

        let pips_x = cw - 320.0;
        for p in 0..a.max_level {
            let px = pips_x + p as f32 * 20.0;
            let filled = p < lv;
            draw_rectangle(
                px,
                y + 28.0,
                14.0,
                14.0,
                if filled {
                    Color::new(0.7, 0.6, 1.0, 1.0)
                } else {
                    Color::new(0.15, 0.15, 0.2, 1.0)
                },
            );
            draw_rectangle_lines(
                px,
                y + 28.0,
                14.0,
                14.0,
                1.0,
                Color::new(0.5, 0.5, 0.6, 1.0),
            );
        }

        if maxed {
            draw_text(
                "MAX",
                cw - 100.0,
                y + 45.0,
                22.0,
                Color::new(0.4, 1.0, 0.4, 1.0),
            );
        } else {
            draw_text(
                &format!("{}c", cost),
                cw - 100.0,
                y + 45.0,
                22.0,
                if affordable {
                    GOLD
                } else {
                    Color::new(0.5, 0.5, 0.5, 1.0)
                },
            );
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        let a = &ABILITIES[*sel];
        let lv = save.ability_levels[*sel];
        if lv < a.max_level {
            let cost = ability_cost(*sel, lv);
            if save.coins >= cost {
                save.coins -= cost;
                save.ability_levels[*sel] += 1;
            }
        }
    }
    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Menu);
    }
    let _ = sh;
    None
}

fn draw_pause() -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.7));
    center_text("PAUSED", sh * 0.35, 64.0, Color::new(1.0, 0.85, 0.3, 1.0));
    center_text(
        "ESC resume  |  Q quit to menu",
        sh * 0.5,
        24.0,
        Color::new(0.7, 0.75, 0.85, 1.0),
    );
    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Play);
    }
    if is_key_pressed(KeyCode::Q) {
        return Some(State::Menu);
    }
    None
}

fn draw_dead(save: &mut Save, score: u32, kills: u32, wave: u32) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.15, 0.0, 0.0, 0.75));
    center_text("YOU DIED", sh * 0.28, 88.0, Color::new(1.0, 0.3, 0.25, 1.0));
    center_text(&format!("SCORE {}", score), sh * 0.42, 32.0, WHITE);
    center_text(&format!("WAVE {}", wave), sh * 0.42 + 40.0, 32.0, WHITE);
    center_text(&format!("KILLS {}", kills), sh * 0.42 + 80.0, 32.0, WHITE);
    center_text(
        &format!("BEST WAVE {}", save.best_wave),
        sh * 0.42 + 130.0,
        26.0,
        GOLD,
    );
    center_text(
        "ENTER to continue",
        sh * 0.72,
        28.0,
        Color::new(0.9, 0.9, 0.5, 1.0),
    );
    if is_key_pressed(KeyCode::Enter) {
        return Some(State::Menu);
    }
    None
}

// ===================== MAIN =====================
fn window_conf() -> Conf {
    Conf {
        window_title: "Deadzone: Zombie Survival".to_owned(),
        window_width: 1280,
        window_height: 720,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut save = Save::load_from_disk();
    let mut state = State::Loading;
    let mut load_t = 0.0f32;

    let mut menu_sel = 0usize;
    let mut shop_sel = 0usize;
    let mut inv_weapon = 0usize;
    let mut inv_slot = 0usize;
    let mut inv_att = 0usize;
    let mut abil_sel = 0usize;
    let mut map_idx = 0usize;

    let mut game = Game::new(&save, map_idx);
    let mut last_mouse: Vec2 = mouse_position().into();
    let mut grabbed = false;
    let mut prev_wave_for_best = 0u32;

    loop {
        let dt = get_frame_time().min(0.05);
        let mp: Vec2 = mouse_position().into();
        let mouse_delta = mp - last_mouse;
        last_mouse = mp;
        let wheel = mouse_wheel();
        let wheel_y = wheel.1;

        clear_background(Color::new(0.02, 0.03, 0.05, 1.0));

        match state {
            State::Loading => {
                load_t += dt;
                let sw = screen_width();
                let sh = screen_height();
                let p = (load_t / 1.6).min(1.0);

                for i in 0..12 {
                    let a = 0.02 + 0.04 * ((i as f32 * 0.7 + load_t * 3.0).sin() * 0.5 + 0.5);
                    draw_rectangle(
                        0.0,
                        i as f32 * sh / 12.0,
                        sw,
                        sh / 12.0,
                        Color::new(0.2, 0.3, 0.5, a),
                    );
                }

                center_text(
                    "DEADZONE",
                    sh * 0.35,
                    84.0,
                    Color::new(1.0, 0.35, 0.25, 1.0),
                );
                center_text(
                    "ZOMBIE SURVIVAL",
                    sh * 0.35 + 50.0,
                    26.0,
                    Color::new(0.7, 0.75, 0.85, 1.0),
                );

                let bw = 500.0;
                let bh = 22.0;
                let bx = sw / 2.0 - bw / 2.0;
                let by = sh * 0.55;
                draw_rectangle(bx, by, bw, bh, Color::new(0.1, 0.12, 0.18, 1.0));
                draw_rectangle(bx, by, bw * p, bh, Color::new(1.0, 0.45, 0.25, 1.0));
                draw_rectangle_lines(bx, by, bw, bh, 1.0, Color::new(0.6, 0.6, 0.7, 1.0));
                center_text(&format!("{:.0}%", p * 100.0), by + 60.0, 22.0, WHITE);

                if load_t > 1.8 {
                    state = State::Menu;
                }
            }

            State::Menu => {
                game.draw_world();
                set_default_camera();
                match draw_menu(&save, &mut menu_sel, &mut map_idx) {
                    Some(MenuCmd::Play) => {
                        game = Game::new(&save, map_idx);
                        prev_wave_for_best = 0;
                        state = State::Play;
                    }
                    Some(MenuCmd::Shop) => state = State::Shop,
                    Some(MenuCmd::Inventory) => state = State::Inventory,
                    Some(MenuCmd::Abilities) => state = State::Abilities,
                    Some(MenuCmd::CycleMap) => {
                        map_idx = (map_idx + 1) % maps().len();
                        game = Game::new(&save, map_idx);
                    }
                    Some(MenuCmd::Quit) => break,
                    None => {}
                }
            }

            State::Shop => {
                if let Some(ns) = draw_shop(&mut save, &mut shop_sel) {
                    state = ns;
                }
            }

            State::Inventory => {
                if let Some(ns) =
                    draw_inventory(&mut save, &mut inv_weapon, &mut inv_slot, &mut inv_att)
                {
                    state = ns;
                }
            }

            State::Abilities => {
                if let Some(ns) = draw_abilities(&mut save, &mut abil_sel) {
                    state = ns;
                }
            }

            State::Play => {
                game.update(dt, mouse_delta, &save, wheel_y);
                game.draw_world();
                set_default_camera();
                game.draw_hud();

                if game.wave > prev_wave_for_best {
                    prev_wave_for_best = game.wave;
                }

                if game.health <= 0.0 {
                    save.coins = game.coins;
                    save.total_kills += game.kills;
                    if prev_wave_for_best > save.best_wave {
                        save.best_wave = prev_wave_for_best;
                    }
                    save.save_to_disk();
                    state = State::Dead;
                } else if is_key_pressed(KeyCode::Escape) {
                    state = State::Pause;
                }
            }

            State::Pause => {
                game.draw_world();
                set_default_camera();
                if let Some(ns) = draw_pause() {
                    if ns == State::Menu {
                        save.coins = game.coins;
                        if prev_wave_for_best > save.best_wave {
                            save.best_wave = prev_wave_for_best;
                        }
                        save.save_to_disk();
                    }
                    state = ns;
                }
            }

            State::Dead => {
                if let Some(ns) = draw_dead(&mut save, game.score, game.kills, game.wave) {
                    state = ns;
                }
            }
        }

        let want_grab = state == State::Play;
        if want_grab != grabbed {
            let _ = set_cursor_grab(want_grab);
            show_mouse(!want_grab);
            grabbed = want_grab;
            last_mouse = mouse_position().into();
        }

        next_frame().await;
    }

    save.save_to_disk();
}
