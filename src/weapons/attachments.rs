use crate::systems::save::Save;

#[derive(Clone, Copy)]
pub struct AttachmentDef {
    pub name: &'static str,
    pub slot: usize,
    pub damage_mult: f32,
    pub spread_mult: f32,
    pub recoil_mult: f32,
    pub reload_mult: f32,
    pub mag_add: i32,
    pub zoom: f32,
    pub price: u32,
    pub desc: &'static str,
}

// Slot indices: 0=optic 1=barrel 2=underbarrel 3=magazine 4=stock
//
// IMPORTANT: Old attachment indices (0-12) must not move — saves persist
// ownership by index. New attachments appended at 13-21.
pub const ATTACHMENTS: [AttachmentDef; 22] = [
    // ============================================================
    // OPTICS (slot 0)
    // ============================================================
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
    // ============================================================
    // BARRELS (slot 1)
    // ============================================================
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
    // ============================================================
    // UNDERBARRELS (slot 2)
    // ============================================================
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
    // ============================================================
    // MAGAZINES (slot 3)
    // ============================================================
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
    // ============================================================
    // STOCKS (slot 4)
    // ============================================================
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
    // ============================================================
    // NEW — OPTICS
    // ============================================================
    AttachmentDef {
        name: "THERMAL",
        slot: 0,
        damage_mult: 1.00,
        spread_mult: 0.72,
        recoil_mult: 1.00,
        reload_mult: 1.05,
        mag_add: 0,
        zoom: 2.20,
        price: 1400,
        desc: "vision through heat, heavy",
    },
    // ============================================================
    // NEW — BARRELS
    // ============================================================
    AttachmentDef {
        name: "MUZZLE BRAKE",
        slot: 1,
        damage_mult: 1.00,
        spread_mult: 0.90,
        recoil_mult: 0.65,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 450,
        desc: "max recoil control",
    },
    AttachmentDef {
        name: "HEAVY BARREL",
        slot: 1,
        damage_mult: 1.20,
        spread_mult: 0.85,
        recoil_mult: 1.15,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 750,
        desc: "max damage, +recoil",
    },
    // ============================================================
    // NEW — UNDERBARRELS
    // ============================================================
    AttachmentDef {
        name: "BIPOD",
        slot: 2,
        damage_mult: 1.00,
        spread_mult: 0.85,
        recoil_mult: 0.50,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 500,
        desc: "elite recoil control",
    },
    AttachmentDef {
        name: "LASER SIGHT",
        slot: 2,
        damage_mult: 1.00,
        spread_mult: 0.80,
        recoil_mult: 0.95,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 400,
        desc: "hipfire accuracy",
    },
    // ============================================================
    // NEW — MAGAZINES
    // ============================================================
    AttachmentDef {
        name: "DRUM MAG",
        slot: 3,
        damage_mult: 1.00,
        spread_mult: 1.00,
        recoil_mult: 1.00,
        reload_mult: 1.35,
        mag_add: 40,
        zoom: 1.00,
        price: 700,
        desc: "+40 mag, slow reload",
    },
    AttachmentDef {
        name: "SPEED LOADER",
        slot: 3,
        damage_mult: 1.00,
        spread_mult: 1.00,
        recoil_mult: 1.00,
        reload_mult: 0.55,
        mag_add: 0,
        zoom: 1.00,
        price: 550,
        desc: "blazing reload",
    },
    // ============================================================
    // NEW — STOCKS
    // ============================================================
    AttachmentDef {
        name: "SKELETON STOCK",
        slot: 4,
        damage_mult: 1.00,
        spread_mult: 0.95,
        recoil_mult: 0.80,
        reload_mult: 1.00,
        mag_add: 0,
        zoom: 1.00,
        price: 300,
        desc: "-recoil",
    },
    AttachmentDef {
        name: "PRECISION STOCK",
        slot: 4,
        damage_mult: 1.00,
        spread_mult: 0.75,
        recoil_mult: 0.70,
        reload_mult: 1.15,
        mag_add: 0,
        zoom: 1.00,
        price: 500,
        desc: "elite accuracy",
    },
];

pub const SLOT_NAMES: [&str; 5] = ["OPTIC", "BARREL", "UNDERBARREL", "MAGAZINE", "STOCK"];

/// Aggregate attachment modifiers for a given weapon.
/// Returns (recoil_mult, spread_mult, reload_mult, damage_mult, mag_bonus, zoom_mult).
pub fn compute_mods(save: &Save, w_idx: usize) -> (f32, f32, f32, f32, i32, f32) {
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
