use macroquad::prelude::*;

use crate::game::camera::look_dir;
use crate::systems::save::Save;
use crate::weapons::attachments::compute_mods;
use crate::weapons::weapon::WEAPONS;

pub const PLAYER_R: f32 = 0.5;
pub const WALK_SPEED: f32 = 6.0;
pub const SPRINT_SPEED: f32 = 9.5;
pub const CROUCH_SPEED: f32 = 3.2;
pub const STAMINA_MAX: f32 = 100.0;
pub const STAMINA_DRAIN: f32 = 24.0;
pub const STAMINA_REGEN: f32 = 20.0;
pub const GRAVITY: f32 = 22.0;
pub const JUMP_VEL: f32 = 7.5;
pub const EYE_STAND: f32 = 1.7;
pub const EYE_CROUCH: f32 = 1.0;

pub struct Player {
    pub pos: Vec3,
    pub vel_y: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub grounded: bool,
    pub double_jump_used: bool,
    pub crouching: bool,
    pub stamina: f32,
    pub health: f32,
    pub max_health: f32,
    pub armor: f32,
    pub bob: f32,
    pub hurt_flash: f32,
    pub shake: f32,
    pub coins: u32,
    pub ability_lv: [u32; 10],
    pub grenade_count: u32,
    pub grenade_cd: f32,
    pub landmine_count: u32,
    pub claymore_count: u32,
    pub nuke_count: u32,
    pub deploy_cd: f32,
    pub equipped: usize,
    pub primary_slot: usize,
    pub secondary_slot: usize,
    pub ammo: u32,
    pub reserve: u32,
    pub ammo_per_weapon: [u32; 9],
    pub reserve_per_weapon: [u32; 9],
    pub reload_t: f32,
    pub cooldown: f32,
    pub recoil: f32,
    pub ads: bool,
    pub hit_mark: f32,
    pub muzzle_flash: f32,
    pub recoil_mult: f32,
    pub spread_mult: f32,
    pub reload_mult: f32,
    pub damage_mult: f32,
    pub mag_bonus: i32,
    pub zoom_mult: f32,
    pub sprint_kick: f32,
    pub land_dip: f32,
    pub strafe_lean: f32,
    pub mouse_sway: Vec2,
    pub fov_deg: f32,
    pub sensitivity: f32,
}

impl Player {
    pub fn new(save: &Save, equipped: usize) -> Self {
        let max_hp = 100.0 + save.ability_levels[3] as f32 * 20.0;
        let (recoil_mult, spread_mult, reload_mult, damage_mult, mag_bonus, zoom_mult) =
            compute_mods(save, equipped);

        let mut ammo_arr = [0u32; 9];
        let mut reserve_arr = [0u32; 9];
        for i in 0..9 {
            if !WEAPONS[i].is_melee && save.owned_weapons[i] {
                let base = WEAPONS[i].mag_size;
                let m = if i == equipped {
                    ((base as i32 + mag_bonus).max(1)) as u32
                } else {
                    base
                };
                ammo_arr[i] = m;
                reserve_arr[i] = m * 4;
            }
        }

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
            coins: save.coins,
            ability_lv: save.ability_levels,
            grenade_count: 3,
            grenade_cd: 0.0,
            landmine_count: 0,
            claymore_count: 0,
            nuke_count: 0,
            deploy_cd: 0.0,
            equipped,
            primary_slot: save.loadout_primary,
            secondary_slot: save.loadout_secondary,
            ammo: ammo_arr[equipped],
            reserve: reserve_arr[equipped],
            ammo_per_weapon: ammo_arr,
            reserve_per_weapon: reserve_arr,
            reload_t: 0.0,
            cooldown: 0.0,
            recoil: 0.0,
            ads: false,
            hit_mark: 0.0,
            muzzle_flash: 0.0,
            recoil_mult,
            spread_mult,
            reload_mult,
            damage_mult,
            mag_bonus,
            zoom_mult,
            sprint_kick: 0.0,
            land_dip: 0.0,
            strafe_lean: 0.0,
            mouse_sway: Vec2::ZERO,
            fov_deg: save.settings.fov,
            sensitivity: save.settings.sensitivity,
        }
    }

    pub fn eye_height(&self) -> f32 {
        if self.crouching {
            EYE_CROUCH
        } else {
            EYE_STAND
        }
    }

    pub fn eye(&self) -> Vec3 {
        let lean_offset = self.strafe_lean * 0.05;
        let dip = -self.land_dip * 0.18;
        vec3(
            self.pos.x + lean_offset,
            self.eye_height() + self.bob.sin() * 0.035 + dip + self.pos.y,
            self.pos.z,
        )
    }

    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let f = look_dir(self.yaw, self.pitch);
        let r = f.cross(Vec3::Y).normalize();
        let u = r.cross(f);
        (f, r, u)
    }

    pub fn current_mag(&self) -> u32 {
        if WEAPONS[self.equipped].is_melee {
            return 0;
        }
        ((WEAPONS[self.equipped].mag_size as i32 + self.mag_bonus).max(1)) as u32
    }

    pub fn current_reload(&self) -> f32 {
        if WEAPONS[self.equipped].is_melee {
            return 0.0;
        }
        WEAPONS[self.equipped].reload_time
            * self.reload_mult
            * (1.0 - self.ability_lv[2] as f32 * 0.10).max(0.3)
    }
}
