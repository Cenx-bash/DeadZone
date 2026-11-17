use macroquad::prelude::*;

use crate::game::Game;
use crate::maps::arena::HALF;
use crate::zombies::enemy::{Enemy, ZombieKind};

impl Game {
    pub(crate) fn spawn_zombie(&mut self, wave: u32) {
        let wave = wave.max(1);
        let w = wave as f32;

        // Spawn position
        let side = rand::gen_range(0, 4);
        let t = rand::gen_range(-HALF + 1.5, HALF - 1.5);
        let edge = HALF - 1.0;
        let (x, z) = match side {
            0 => (t, -edge),
            1 => (t, edge),
            2 => (-edge, t),
            _ => (edge, t),
        };

        // Spawn probabilities
        let boss_chance = if wave >= 5 {
            (0.015 + w * 0.003).min(0.15)
        } else {
            0.0
        };
        let tank_chance = if wave >= 3 {
            (0.05 + w * 0.005).min(0.18)
        } else {
            0.0
        };
        let spitter_chance = if wave >= 3 {
            (0.08 + w * 0.004).min(0.18)
        } else {
            0.0
        };
        let runner_chance = if wave >= 2 {
            (0.12 + w * 0.008).min(0.28)
        } else {
            0.0
        };
        let crawler_chance = if wave >= 2 {
            (0.08 + w * 0.004).min(0.16)
        } else {
            0.0
        };

        // Roll
        let roll = rand::gen_range(0.0, 1.0);
        let mut threshold = boss_chance;
        let kind = if roll < threshold {
            ZombieKind::Boss
        } else {
            threshold += tank_chance;
            if roll < threshold {
                ZombieKind::Tank
            } else {
                threshold += spitter_chance;
                if roll < threshold {
                    ZombieKind::Spitter
                } else {
                    threshold += runner_chance;
                    if roll < threshold {
                        ZombieKind::Runner
                    } else {
                        threshold += crawler_chance;
                        if roll < threshold {
                            ZombieKind::Crawler
                        } else {
                            ZombieKind::Walker
                        }
                    }
                }
            }
        };

        let hp = kind.hp(wave);

        // First-hit grace scales with wave but never goes below the
        // steady-state attack cooldown (0.65s).
        let attack_cd = (0.65 - w * 0.008).max(0.65);
        // First-shot delay is a spawn warm-up, never shorter than the
        // steady-state spitter cooldown (1.5s).
        let shoot_cd = (2.0 - w * 0.025).max(1.5);

        self.enemies.push(Enemy {
            pos: vec3(x, 0.0, z),
            kind,
            hp,
            max_hp: hp,
            flash: 0.0,
            atk_cd: attack_cd,
            phase: rand::gen_range(0.0, 6.0),
            shoot_cd,
            is_boss: kind == ZombieKind::Boss,
        });
    }
}
