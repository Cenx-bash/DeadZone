use macroquad::prelude::*;

use crate::game::camera::SENS;
use crate::game::player::{
    CROUCH_SPEED, GRAVITY, JUMP_VEL, PLAYER_R, SPRINT_SPEED, STAMINA_DRAIN, STAMINA_MAX,
    STAMINA_REGEN, WALK_SPEED,
};
use crate::game::Game;
use crate::systems::collision::push_out;
use crate::systems::save::Save;

impl Game {
    pub(crate) fn update_look(&mut self, dt: f32, look: Vec2) {
        let sens = self.player.sensitivity;
        self.player.yaw += look.x * sens;
        self.player.pitch -= look.y * sens;
        if is_key_down(KeyCode::Left) {
            self.player.yaw -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.player.yaw += 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            self.player.pitch += 1.5 * dt;
        }
        if is_key_down(KeyCode::Down) {
            self.player.pitch -= 1.5 * dt;
        }
        self.player.pitch = self.player.pitch.clamp(-1.5, 1.5);
        self.player.ads = is_mouse_button_down(MouseButton::Right);
        self.player.mouse_sway.x = (self.player.mouse_sway.x + look.x * 0.03).clamp(-0.5, 0.5);
        self.player.mouse_sway.y = (self.player.mouse_sway.y + look.y * 0.03).clamp(-0.5, 0.5);
        let _ = SENS;
    }

    pub(crate) fn update_weapon_switch(&mut self, save: &Save, wheel_y: f32) {
        if is_key_pressed(KeyCode::Key1) && save.owned_weapons[save.loadout_primary] {
            self.switch_weapon(save.loadout_primary, save);
        }
        if is_key_pressed(KeyCode::Key2) && save.owned_weapons[save.loadout_secondary] {
            self.switch_weapon(save.loadout_secondary, save);
        }
        if is_key_pressed(KeyCode::Key3) {
            self.switch_weapon(1, save);
        }
        if wheel_y.abs() > 0.1 {
            let dir = if wheel_y > 0.0 { -1 } else { 1 };
            self.cycle_loadout_weapon(save, dir);
        }
    }

    pub(crate) fn update_move(&mut self, dt: f32) {
        let fwd = vec3(self.player.yaw.cos(), 0.0, self.player.yaw.sin());
        let right = vec3(-self.player.yaw.sin(), 0.0, self.player.yaw.cos());

        self.player.crouching = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::C);
        let sprinting = is_key_down(KeyCode::LeftShift)
            && !self.player.crouching
            && self.player.stamina > 0.0
            && self.player.grounded;

        let speed = if self.player.crouching {
            CROUCH_SPEED
        } else if sprinting {
            SPRINT_SPEED * (1.0 + self.player.ability_lv[6] as f32 * 0.10)
        } else {
            WALK_SPEED * (1.0 + self.player.ability_lv[0] as f32 * 0.08)
        };

        if sprinting {
            self.player.stamina = (self.player.stamina - STAMINA_DRAIN * dt).max(0.0);
        } else {
            self.player.stamina = (self.player.stamina + STAMINA_REGEN * dt).min(STAMINA_MAX);
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

        let moving = mv.length() > 0.0;
        if moving {
            mv = mv.normalize() * speed;
            self.player.bob += dt * 10.0;
        }
        self.player.pos += mv * dt;
        push_out(&mut self.player.pos, PLAYER_R, &self.pillars, &self.crates);

        let target_kick = if sprinting && moving { 1.0 } else { 0.0 };
        self.player.sprint_kick += (target_kick - self.player.sprint_kick) * dt * 8.0;

        let strafe = (if is_key_down(KeyCode::D) { 1.0 } else { 0.0 })
            - (if is_key_down(KeyCode::A) { 1.0 } else { 0.0 });
        let lean_target = strafe * if self.player.crouching { 0.3 } else { 1.0 };
        self.player.strafe_lean += (lean_target - self.player.strafe_lean) * dt * 10.0;

        self.player.land_dip = (self.player.land_dip - dt * 5.0).max(0.0);
        self.player.mouse_sway *= (1.0 - dt * 8.0).max(0.0);
    }

    pub(crate) fn update_jump(&mut self, dt: f32) {
        let was_grounded = self.player.grounded;

        if is_key_pressed(KeyCode::Space) {
            if self.player.grounded {
                self.player.vel_y = JUMP_VEL;
                self.player.grounded = false;
                self.player.double_jump_used = false;
            } else if self.player.ability_lv[1] > 0 && !self.player.double_jump_used {
                self.player.vel_y = JUMP_VEL * 0.85;
                self.player.double_jump_used = true;
            }
        }
        self.player.vel_y -= GRAVITY * dt;
        self.player.pos.y += self.player.vel_y * dt;
        if self.player.pos.y <= 0.0 {
            self.player.pos.y = 0.0;
            self.player.vel_y = 0.0;
            self.player.grounded = true;
            self.player.double_jump_used = false;
            if !was_grounded && self.player.land_dip < 0.1 {
                self.player.land_dip = 1.0;
            }
        }
    }
}
