use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ZombieKind {
    // --- Original roster ---
    Walker,
    Runner,
    Tank,
    Crawler,
    Spitter,
    Boss,
    // --- New ---
    Bloater,    // slow, swollen; bursts into a toxic cloud on death
    Screamer,   // buffs the speed of every zombie near it
    Charger,    // winds up, then bulldozes in a straight line
    Leaper,     // pounces from range
    Stalker,    // flickers in and out of sight
    Juggernaut, // heavy plating; shrugs off frontal fire
    Hive,       // living nest that births crawlers
}

/// Special behavior hooks. Your AI/update code can match on these;
/// every number here is a tuning value, not a hard rule.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Ability {
    None,
    /// Ranged acid attack.
    Spit,
    /// Explodes on death.
    Explode {
        radius: f32,
        damage: f32,
    },
    /// Aura: zombies within `radius` move `speed_mult` times faster.
    Scream {
        radius: f32,
        speed_mult: f32,
    },
    /// Pauses `windup` seconds, then runs `speed_mult` times faster for `duration`.
    Charge {
        windup: f32,
        speed_mult: f32,
        duration: f32,
    },
    /// Jumps at the player when within `range`.
    Leap {
        range: f32,
        cooldown: f32,
    },
    /// Fades in and out; `visible_frac` is how much of the cycle it can be seen.
    Cloak {
        visible_frac: f32,
    },
    /// Damage taken from the front is multiplied by `front_mult` (lower = tougher).
    Armor {
        front_mult: f32,
    },
    /// Spawns `count` crawlers every `interval` seconds.
    Summon {
        count: u32,
        interval: f32,
    },
}

/// Proportions for the renderer. 1.0 = "average zombie".
/// Multiply your head/torso/arm/leg dimensions by these.
#[derive(Clone, Copy, Debug)]
pub struct Look {
    pub head: f32,      // head size
    pub shoulders: f32, // torso width
    pub arms: f32,      // arm length
    pub legs: f32,      // leg length
    pub hunch: f32,     // forward lean in radians-ish (0 = upright)
    pub bulk: f32,      // overall girth multiplier
}

impl ZombieKind {
    pub const ALL: [ZombieKind; 13] = [
        ZombieKind::Walker,
        ZombieKind::Runner,
        ZombieKind::Tank,
        ZombieKind::Crawler,
        ZombieKind::Spitter,
        ZombieKind::Boss,
        ZombieKind::Bloater,
        ZombieKind::Screamer,
        ZombieKind::Charger,
        ZombieKind::Leaper,
        ZombieKind::Stalker,
        ZombieKind::Juggernaut,
        ZombieKind::Hive,
    ];

    // ------------------------------------------------------------
    // Stats
    // ------------------------------------------------------------
    pub fn hp(self, wave: u32) -> f32 {
        let base = match self {
            ZombieKind::Walker => 30.0,
            ZombieKind::Runner => 22.0,
            ZombieKind::Tank => 180.0,
            ZombieKind::Crawler => 18.0,
            ZombieKind::Spitter => 45.0,
            ZombieKind::Boss => 900.0,
            ZombieKind::Bloater => 70.0,
            ZombieKind::Screamer => 28.0,
            ZombieKind::Charger => 90.0,
            ZombieKind::Leaper => 26.0,
            ZombieKind::Stalker => 35.0,
            ZombieKind::Juggernaut => 320.0,
            ZombieKind::Hive => 140.0,
        };
        base * (1.0 + wave as f32 * 0.06)
    }

    pub fn speed(self) -> f32 {
        match self {
            ZombieKind::Walker => 2.2,
            ZombieKind::Runner => 5.8,
            ZombieKind::Tank => 1.6,
            ZombieKind::Crawler => 3.6,
            ZombieKind::Spitter => 2.0,
            ZombieKind::Boss => 2.6,
            ZombieKind::Bloater => 1.4,
            ZombieKind::Screamer => 3.0,
            ZombieKind::Charger => 2.4,
            ZombieKind::Leaper => 3.2,
            ZombieKind::Stalker => 4.2,
            ZombieKind::Juggernaut => 1.3,
            ZombieKind::Hive => 1.2,
        }
    }

    pub fn radius(self) -> f32 {
        match self {
            ZombieKind::Walker => 0.5,
            ZombieKind::Runner => 0.4,
            ZombieKind::Tank => 0.9,
            ZombieKind::Crawler => 0.55,
            ZombieKind::Spitter => 0.5,
            ZombieKind::Boss => 1.4,
            ZombieKind::Bloater => 0.8,
            ZombieKind::Screamer => 0.45,
            ZombieKind::Charger => 0.7,
            ZombieKind::Leaper => 0.45,
            ZombieKind::Stalker => 0.45,
            ZombieKind::Juggernaut => 1.0,
            ZombieKind::Hive => 0.9,
        }
    }

    pub fn height(self) -> f32 {
        match self {
            ZombieKind::Walker => 1.8,
            ZombieKind::Runner => 1.4,
            ZombieKind::Tank => 2.8,
            ZombieKind::Crawler => 0.7,
            ZombieKind::Spitter => 1.9,
            ZombieKind::Boss => 4.2,
            ZombieKind::Bloater => 2.2,
            ZombieKind::Screamer => 1.75,
            ZombieKind::Charger => 2.2,
            ZombieKind::Leaper => 1.5,
            ZombieKind::Stalker => 1.8,
            ZombieKind::Juggernaut => 3.0,
            ZombieKind::Hive => 2.4,
        }
    }

    pub fn damage(self) -> f32 {
        match self {
            ZombieKind::Walker => 10.0,
            ZombieKind::Runner => 8.0,
            ZombieKind::Tank => 28.0,
            ZombieKind::Crawler => 6.0,
            ZombieKind::Spitter => 0.0,
            ZombieKind::Boss => 45.0,
            ZombieKind::Bloater => 12.0,
            ZombieKind::Screamer => 5.0,
            ZombieKind::Charger => 22.0,
            ZombieKind::Leaper => 14.0,
            ZombieKind::Stalker => 16.0,
            ZombieKind::Juggernaut => 35.0,
            ZombieKind::Hive => 0.0,
        }
    }

    pub fn score(self) -> u32 {
        match self {
            ZombieKind::Walker => 10,
            ZombieKind::Runner => 15,
            ZombieKind::Tank => 60,
            ZombieKind::Crawler => 12,
            ZombieKind::Spitter => 25,
            ZombieKind::Boss => 500,
            ZombieKind::Bloater => 30,
            ZombieKind::Screamer => 30,
            ZombieKind::Charger => 40,
            ZombieKind::Leaper => 20,
            ZombieKind::Stalker => 28,
            ZombieKind::Juggernaut => 100,
            ZombieKind::Hive => 80,
        }
    }

    // ------------------------------------------------------------
    // Abilities
    // ------------------------------------------------------------
    pub fn ability(self) -> Ability {
        match self {
            ZombieKind::Spitter => Ability::Spit,
            ZombieKind::Bloater => Ability::Explode {
                radius: 3.5,
                damage: 25.0,
            },
            ZombieKind::Screamer => Ability::Scream {
                radius: 8.0,
                speed_mult: 1.35,
            },
            ZombieKind::Charger => Ability::Charge {
                windup: 0.9,
                speed_mult: 3.3,
                duration: 1.6,
            },
            ZombieKind::Leaper => Ability::Leap {
                range: 9.0,
                cooldown: 3.5,
            },
            ZombieKind::Stalker => Ability::Cloak { visible_frac: 0.45 },
            ZombieKind::Juggernaut => Ability::Armor { front_mult: 0.4 },
            ZombieKind::Hive => Ability::Summon {
                count: 3,
                interval: 6.0,
            },
            _ => Ability::None,
        }
    }

    // ------------------------------------------------------------
    // Design: names, palette, proportions
    // ------------------------------------------------------------
    pub fn name(self) -> &'static str {
        match self {
            ZombieKind::Walker => "WALKER",
            ZombieKind::Runner => "RUNNER",
            ZombieKind::Tank => "TANK",
            ZombieKind::Crawler => "CRAWLER",
            ZombieKind::Spitter => "SPITTER",
            ZombieKind::Boss => "ABOMINATION",
            ZombieKind::Bloater => "BLOATER",
            ZombieKind::Screamer => "SCREAMER",
            ZombieKind::Charger => "CHARGER",
            ZombieKind::Leaper => "LEAPER",
            ZombieKind::Stalker => "STALKER",
            ZombieKind::Juggernaut => "JUGGERNAUT",
            ZombieKind::Hive => "HIVE",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            ZombieKind::Walker => "Slow, relentless, everywhere.",
            ZombieKind::Runner => "Fast and fragile. Shoot first.",
            ZombieKind::Tank => "Heavy hitter with a lot of meat.",
            ZombieKind::Crawler => "Low to the ground, hard to hit.",
            ZombieKind::Spitter => "Keeps its distance and spits acid.",
            ZombieKind::Boss => "Everything you fear, in one body.",
            ZombieKind::Bloater => "Pop it from afar. Do not pop it close.",
            ZombieKind::Screamer => "Kill it first or the horde speeds up.",
            ZombieKind::Charger => "Watch the windup, then sidestep.",
            ZombieKind::Leaper => "Pounces. Keep moving.",
            ZombieKind::Stalker => "You only see it when it wants you to.",
            ZombieKind::Juggernaut => "Plated front. Flank it.",
            ZombieKind::Hive => "Kill the nest before it fills the map.",
        }
    }

    /// Main skin / body color.
    pub fn color(self) -> Color {
        match self {
            ZombieKind::Walker => Color::new(0.55, 0.47, 0.38, 1.0),
            ZombieKind::Runner => Color::new(0.82, 0.48, 0.30, 1.0),
            ZombieKind::Tank => Color::new(0.45, 0.27, 0.50, 1.0),
            ZombieKind::Crawler => Color::new(0.50, 0.56, 0.30, 1.0),
            ZombieKind::Spitter => Color::new(0.38, 0.78, 0.42, 1.0),
            ZombieKind::Boss => Color::new(0.80, 0.14, 0.14, 1.0),
            ZombieKind::Bloater => Color::new(0.62, 0.68, 0.32, 1.0),
            ZombieKind::Screamer => Color::new(0.72, 0.72, 0.80, 1.0),
            ZombieKind::Charger => Color::new(0.60, 0.30, 0.22, 1.0),
            ZombieKind::Leaper => Color::new(0.35, 0.55, 0.60, 1.0),
            ZombieKind::Stalker => Color::new(0.22, 0.24, 0.32, 1.0),
            ZombieKind::Juggernaut => Color::new(0.42, 0.44, 0.50, 1.0),
            ZombieKind::Hive => Color::new(0.58, 0.30, 0.38, 1.0),
        }
    }

    /// Secondary color: clothing, armor plates, tumors, wounds.
    pub fn accent(self) -> Color {
        match self {
            ZombieKind::Walker => Color::new(0.28, 0.30, 0.38, 1.0), // torn jeans/shirt
            ZombieKind::Runner => Color::new(0.20, 0.20, 0.24, 1.0), // dark tracksuit
            ZombieKind::Tank => Color::new(0.75, 0.30, 0.25, 1.0),   // exposed muscle
            ZombieKind::Crawler => Color::new(0.30, 0.15, 0.12, 1.0), // dried blood
            ZombieKind::Spitter => Color::new(0.85, 0.95, 0.30, 1.0), // acid sacs
            ZombieKind::Boss => Color::new(0.25, 0.05, 0.05, 1.0),   // dark veins
            ZombieKind::Bloater => Color::new(0.85, 0.80, 0.35, 1.0), // pus-yellow boils
            ZombieKind::Screamer => Color::new(0.55, 0.12, 0.18, 1.0), // torn throat
            ZombieKind::Charger => Color::new(0.75, 0.75, 0.70, 1.0), // bone horn plate
            ZombieKind::Leaper => Color::new(0.85, 0.80, 0.55, 1.0), // bone spurs
            ZombieKind::Stalker => Color::new(0.45, 0.55, 0.85, 1.0), // faint blue sheen
            ZombieKind::Juggernaut => Color::new(0.65, 0.55, 0.30, 1.0), // riveted plates
            ZombieKind::Hive => Color::new(0.85, 0.55, 0.60, 1.0),   // egg sacs
        }
    }

    /// Eye color (also used for glow effects).
    pub fn eye(self) -> Color {
        match self {
            ZombieKind::Walker => Color::new(0.95, 0.90, 0.60, 1.0),
            ZombieKind::Runner => Color::new(1.00, 0.35, 0.20, 1.0),
            ZombieKind::Tank => Color::new(1.00, 0.55, 0.20, 1.0),
            ZombieKind::Crawler => Color::new(0.90, 1.00, 0.40, 1.0),
            ZombieKind::Spitter => Color::new(0.50, 1.00, 0.30, 1.0),
            ZombieKind::Boss => Color::new(1.00, 0.85, 0.20, 1.0),
            ZombieKind::Bloater => Color::new(0.95, 0.95, 0.40, 1.0),
            ZombieKind::Screamer => Color::new(1.00, 0.30, 0.50, 1.0),
            ZombieKind::Charger => Color::new(1.00, 0.20, 0.10, 1.0),
            ZombieKind::Leaper => Color::new(0.40, 1.00, 0.90, 1.0),
            ZombieKind::Stalker => Color::new(0.50, 0.80, 1.00, 1.0),
            ZombieKind::Juggernaut => Color::new(1.00, 0.30, 0.15, 1.0),
            ZombieKind::Hive => Color::new(1.00, 0.60, 0.70, 1.0),
        }
    }

    /// Whether the eyes / accent bits should be drawn bright (unaffected by fog).
    pub fn glows(self) -> bool {
        matches!(
            self,
            ZombieKind::Spitter
                | ZombieKind::Boss
                | ZombieKind::Stalker
                | ZombieKind::Screamer
                | ZombieKind::Charger
        )
    }

    /// Body proportions for the renderer.
    pub fn look(self) -> Look {
        let l = |head, shoulders, arms, legs, hunch, bulk| Look {
            head,
            shoulders,
            arms,
            legs,
            hunch,
            bulk,
        };
        match self {
            ZombieKind::Walker => l(1.0, 1.0, 1.1, 1.0, 0.25, 1.0),
            ZombieKind::Runner => l(0.9, 0.8, 1.2, 1.1, 0.45, 0.8),
            ZombieKind::Tank => l(0.75, 1.5, 1.2, 0.85, 0.15, 1.4),
            ZombieKind::Crawler => l(1.1, 0.9, 1.4, 0.4, 0.9, 0.9),
            ZombieKind::Spitter => l(1.1, 0.85, 1.0, 1.0, 0.35, 0.9),
            ZombieKind::Boss => l(0.8, 1.7, 1.5, 1.0, 0.2, 1.6),
            ZombieKind::Bloater => l(0.65, 1.3, 0.8, 0.7, 0.1, 1.9),
            ZombieKind::Screamer => l(1.3, 0.75, 1.0, 1.0, 0.0, 0.8),
            ZombieKind::Charger => l(0.7, 1.6, 1.0, 0.9, 0.5, 1.3),
            ZombieKind::Leaper => l(0.95, 0.8, 1.5, 1.2, 0.7, 0.8),
            ZombieKind::Stalker => l(0.9, 0.7, 1.3, 1.1, 0.3, 0.7),
            ZombieKind::Juggernaut => l(0.7, 1.7, 1.1, 0.9, 0.1, 1.6),
            ZombieKind::Hive => l(0.6, 1.4, 0.7, 0.6, 0.0, 1.8),
        }
    }

    // ------------------------------------------------------------
    // Spawning
    // ------------------------------------------------------------

    /// First wave this kind can appear in. Boss is spawned by the wave
    /// logic itself (e.g. every 5th wave), so it has no random unlock.
    pub fn unlock_wave(self) -> u32 {
        match self {
            ZombieKind::Walker => 1,
            ZombieKind::Crawler => 2,
            ZombieKind::Runner => 3,
            ZombieKind::Spitter => 4,
            ZombieKind::Leaper => 5,
            ZombieKind::Bloater => 6,
            ZombieKind::Screamer => 7,
            ZombieKind::Tank => 8,
            ZombieKind::Stalker => 9,
            ZombieKind::Charger => 10,
            ZombieKind::Hive => 12,
            ZombieKind::Juggernaut => 14,
            ZombieKind::Boss => u32::MAX,
        }
    }

    /// Relative spawn weight for a given wave (0 = cannot spawn yet).
    /// Walkers fade slightly as the roster grows so later waves feel varied.
    pub fn spawn_weight(self, wave: u32) -> f32 {
        if wave < self.unlock_wave() {
            return 0.0;
        }
        let since = (wave - self.unlock_wave()) as f32;
        let ramp = (0.5 + since * 0.15).min(1.0); // newly unlocked kinds start rarer
        let base = match self {
            ZombieKind::Walker => 10.0 - (wave as f32 * 0.25).min(5.0),
            ZombieKind::Crawler => 4.0,
            ZombieKind::Runner => 5.0,
            ZombieKind::Spitter => 3.0,
            ZombieKind::Leaper => 3.0,
            ZombieKind::Bloater => 2.5,
            ZombieKind::Screamer => 1.5,
            ZombieKind::Tank => 2.0,
            ZombieKind::Stalker => 2.0,
            ZombieKind::Charger => 1.5,
            ZombieKind::Hive => 0.8,
            ZombieKind::Juggernaut => 0.8,
            ZombieKind::Boss => 0.0,
        };
        base * ramp
    }

    /// Pick a kind for this wave. `roll` is a random number in 0.0..1.0
    /// (e.g. `rand::gen_range(0.0, 1.0)`).
    pub fn pick_for_wave(wave: u32, roll: f32) -> ZombieKind {
        let total: f32 = Self::ALL.iter().map(|k| k.spawn_weight(wave)).sum();
        if total <= 0.0 {
            return ZombieKind::Walker;
        }
        let mut target = roll.clamp(0.0, 0.9999) * total;
        for k in Self::ALL {
            let w = k.spawn_weight(wave);
            if target < w {
                return k;
            }
            target -= w;
        }
        ZombieKind::Walker
    }
}

pub struct Enemy {
    pub pos: Vec3,
    pub kind: ZombieKind,
    pub hp: f32,
    pub max_hp: f32,
    pub flash: f32,
    pub atk_cd: f32,
    pub phase: f32,
    pub shoot_cd: f32,
    pub is_boss: bool,
}

impl Enemy {
    /// Convenience constructor so spawn code doesn't have to repeat
    /// the stat lookups.
    pub fn new(kind: ZombieKind, pos: Vec3, wave: u32) -> Self {
        let hp = kind.hp(wave);
        Enemy {
            pos,
            kind,
            hp,
            max_hp: hp,
            flash: 0.0,
            atk_cd: 0.0,
            phase: 0.0,
            shoot_cd: 0.0,
            is_boss: kind == ZombieKind::Boss,
        }
    }
}
