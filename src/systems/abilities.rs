pub struct AbilityDef {
    pub name: &'static str,
    pub desc: &'static str,
    pub max_level: u32,
    pub base_cost: u32,
}

pub const ABILITIES: [AbilityDef; 10] = [
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

pub fn ability_cost(idx: usize, level: u32) -> u32 {
    ABILITIES[idx].base_cost * (level + 1) * (level + 1)
}
