use crate::systems::abilities::ABILITIES;
use crate::systems::settings::Settings;
use crate::weapons::attachments::ATTACHMENTS;
use crate::weapons::weapon::{WeaponCategory, WEAPONS};

pub const SAVE_PATH: &str = "fps_save.dat";

#[derive(Clone)]
pub struct Save {
    pub coins: u32,
    pub owned_weapons: [bool; 9],
    pub owned_attachments: [bool; 22],
    pub ability_levels: [u32; 10],
    pub equipped_weapon: usize,
    pub loadout_primary: usize,
    pub loadout_secondary: usize,
    pub weapon_attachments: [[i32; 5]; 9],
    pub best_wave: u32,
    pub total_kills: u32,
    pub settings: Settings,
}

impl Save {
    pub fn fresh() -> Self {
        let mut owned = [false; 9];
        owned[0] = true;
        owned[1] = true;
        owned[2] = true;
        Self {
            coins: 0,
            owned_weapons: owned,
            owned_attachments: [false; 22],
            ability_levels: [0; 10],
            equipped_weapon: 2,
            loadout_primary: 2,
            loadout_secondary: 0,
            weapon_attachments: [[-1; 5]; 9],
            best_wave: 0,
            total_kills: 0,
            settings: Settings::default(),
        }
    }

    pub fn save_to_disk(&self) {
        let mut s = String::new();
        s.push_str(&format!("coins={}\n", self.coins));
        s.push_str(&format!("equipped={}\n", self.equipped_weapon));
        s.push_str(&format!("loadout_primary={}\n", self.loadout_primary));
        s.push_str(&format!("loadout_secondary={}\n", self.loadout_secondary));
        s.push_str(&format!("best_wave={}\n", self.best_wave));
        s.push_str(&format!("total_kills={}\n", self.total_kills));
        s.push_str(&format!("volume={}\n", self.settings.master_volume));
        s.push_str(&format!("sensitivity={}\n", self.settings.sensitivity));
        s.push_str(&format!("fov={}\n", self.settings.fov));
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

    pub fn load_from_disk() -> Self {
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
                "loadout_primary" => {
                    if let Ok(v) = val.parse::<usize>() {
                        if v < 9 && WeaponCategory::of(v) == WeaponCategory::Primary {
                            out.loadout_primary = v;
                        }
                    }
                }
                "loadout_secondary" => {
                    if let Ok(v) = val.parse::<usize>() {
                        if v < 9 && WeaponCategory::of(v) == WeaponCategory::Secondary {
                            out.loadout_secondary = v;
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
                "volume" => {
                    if let Ok(v) = val.parse::<f32>() {
                        out.settings.master_volume = v.clamp(0.0, 1.0);
                    }
                }
                "sensitivity" => {
                    if let Ok(v) = val.parse::<f32>() {
                        out.settings.sensitivity = v.clamp(0.0005, 0.015);
                    }
                }
                "fov" => {
                    if let Ok(v) = val.parse::<f32>() {
                        out.settings.fov = v.clamp(60.0, 110.0);
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
                        if i < 22 {
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
                                        let parsed: i32 = p.parse().unwrap_or(-1);
                                        let valid = parsed >= 0
                                            && (parsed as usize) < ATTACHMENTS.len()
                                            && ATTACHMENTS[parsed as usize].slot == sl;
                                        out.weapon_attachments[w][sl] =
                                            if valid { parsed } else { -1 };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        out.owned_weapons[0] = true;
        out.owned_weapons[1] = true;

        for i in 0..10 {
            let max = ABILITIES[i].max_level;
            if out.ability_levels[i] > max {
                out.ability_levels[i] = max;
            }
        }
        if out.best_wave > 10_000 {
            out.best_wave = 10_000;
        }

        if !out.owned_weapons[out.loadout_primary]
            || WeaponCategory::of(out.loadout_primary) != WeaponCategory::Primary
        {
            out.loadout_primary = (0..9)
                .find(|&i| WeaponCategory::of(i) == WeaponCategory::Primary && out.owned_weapons[i])
                .unwrap_or(2);
        }
        if !out.owned_weapons[out.loadout_secondary]
            || WeaponCategory::of(out.loadout_secondary) != WeaponCategory::Secondary
        {
            out.loadout_secondary = (0..9)
                .find(|&i| {
                    WeaponCategory::of(i) == WeaponCategory::Secondary && out.owned_weapons[i]
                })
                .unwrap_or(0);
        }
        if out.equipped_weapon >= 9 || !out.owned_weapons[out.equipped_weapon] {
            out.equipped_weapon = out.loadout_primary;
        }

        let _ = WEAPONS;
        out
    }
}
