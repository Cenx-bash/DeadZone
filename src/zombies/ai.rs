use macroquad::prelude::*;

use crate::game::player::{EYE_STAND, PLAYER_R};
use crate::game::Game;
use crate::maps::arena::{Crate, Pillar, HALF, PILLAR_R};
use crate::systems::collision::push_out;
use crate::zombies::enemy::ZombieKind;

pub struct Projectile {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub damage: f32,
}

fn point_in_pillar(pos: Vec3, p: &Pillar) -> bool {
    let d = vec2(pos.x - p.x, pos.z - p.z);
    d.length() < PILLAR_R
}

fn point_in_crate(pos: Vec3, c: &Crate) -> bool {
    c.hp > 0.0
        && pos.x >= c.x - c.hw
        && pos.x <= c.x + c.hw
        && pos.z >= c.z - c.hd
        && pos.z <= c.z + c.hd
        && pos.y <= c.h
}

fn projectile_hits_world(pos: Vec3, pillars: &[Pillar], crates: &[Crate]) -> bool {
    if pos.x.abs() > HALF || pos.z.abs() > HALF {
        return true;
    }
    for p in pillars {
        if point_in_pillar(pos, p) {
            return true;
        }
    }
    for c in crates {
        if point_in_crate(pos, c) {
            return true;
        }
    }
    false
}

impl Game {
    pub(crate) fn update_enemies(&mut self, dt: f32) -> f32 {
        let target = self.player.pos;
        let mut damage_taken = 0.0;

        let mut spit_requests: Vec<(Vec3, Vec3, f32, f32)> = Vec::new();

        // Balance
        const SPITTER_RANGE: f32 = 21.0;
        const SPITTER_MIN_DISTANCE: f32 = 6.0;
        const SPITTER_MAX_DISTANCE: f32 = 12.0;

        const SPITTER_DAMAGE: f32 = 12.0;
        const SPITTER_SPEED: f32 = 17.0;
        const SPITTER_COOLDOWN: f32 = 1.5;

        const NORMAL_ATTACK_COOLDOWN: f32 = 0.65;
        const BOSS_ATTACK_COOLDOWN: f32 = 0.5;
        const ENRAGED_ATTACK_COOLDOWN: f32 = 0.4;

        for (index, e) in self.enemies.iter_mut().enumerate() {
            let dx = target.x - e.pos.x;
            let dz = target.z - e.pos.z;
            let dist = (dx * dx + dz * dz).sqrt();
            let stop = e.kind.radius() + PLAYER_R + 0.3;

            let dir = if dist > 0.001 {
                vec3(dx / dist, 0.0, dz / dist)
            } else {
                Vec3::ZERO
            };
            let strafe = vec3(-dir.z, 0.0, dir.x);

            let enraged = e.is_boss && e.hp < e.max_hp * 0.5;

            // ===== SPITTER =====
            if e.kind == ZombieKind::Spitter {
                let desired_distance = if enraged { 9.0 } else { 10.0 };

                if dist > SPITTER_MAX_DISTANCE {
                    e.pos += dir * e.kind.speed() * 1.5 * dt;
                } else if dist < SPITTER_MIN_DISTANCE {
                    e.pos -= dir * e.kind.speed() * 1.8 * dt;
                } else {
                    let direction = if index % 2 == 0 { 1.0 } else { -1.0 };
                    let orbit_speed = if enraged { 1.5 } else { 1.0 };
                    let distance_adjustment = if dist > desired_distance {
                        dir * 0.3
                    } else {
                        -dir * 0.3
                    };
                    e.pos += (strafe * direction + distance_adjustment)
                        * e.kind.speed()
                        * orbit_speed
                        * dt;
                }

                e.shoot_cd -= dt;
                if e.shoot_cd <= 0.0 && dist < SPITTER_RANGE {
                    let from = vec3(e.pos.x, e.kind.height() * 0.7, e.pos.z);
                    let aim_y = EYE_STAND * 0.5;

                    for offset in [-1.0_f32, 0.0, 1.0] {
                        let to = vec3(
                            target.x + strafe.x * offset,
                            aim_y,
                            target.z + strafe.z * offset,
                        );
                        spit_requests.push((from, to, SPITTER_DAMAGE, SPITTER_SPEED));
                    }
                    e.shoot_cd = SPITTER_COOLDOWN;
                }
            } else {
                // ===== MELEE =====
                let approach_bonus = if dist < 5.0 {
                    1.5
                } else if dist < 10.0 {
                    1.25
                } else {
                    1.0
                };
                let speed_mult = approach_bonus * if enraged { 1.5 } else { 1.0 };

                let flank_direction = if index % 2 == 0 { 1.0 } else { -1.0 };
                let flank_strength = if dist > stop + 1.0 { 0.35 } else { 0.0 };
                let movement = dir + strafe * flank_direction * flank_strength;

                if dist > stop && dist > 0.001 {
                    e.pos += movement.normalize() * e.kind.speed() * speed_mult * dt;
                }

                if dist < stop + 0.8 {
                    e.atk_cd -= dt;
                    if e.atk_cd <= 0.0 {
                        let damage_mult = if enraged {
                            1.4
                        } else if e.is_boss {
                            1.2
                        } else {
                            1.1
                        };
                        damage_taken += e.kind.damage() * damage_mult;
                        e.atk_cd = if enraged {
                            ENRAGED_ATTACK_COOLDOWN
                        } else if e.is_boss {
                            BOSS_ATTACK_COOLDOWN
                        } else {
                            NORMAL_ATTACK_COOLDOWN
                        };
                    }
                }
            }

            e.phase += dt * e.kind.speed() * 3.0;
            e.flash = (e.flash - dt).max(0.0);
            push_out(&mut e.pos, e.kind.radius(), &self.pillars, &self.crates);
        }

        // Spawn projectiles
        for (from, to, damage, speed) in spit_requests {
            let diff = to - from;
            if diff.length_squared() > 0.0001 {
                let direction = diff.normalize();
                self.projectiles.push(Projectile {
                    pos: from,
                    vel: direction * speed,
                    life: 2.5,
                    damage,
                });
            }
        }

        // Enemy separation
        let count = self.enemies.len();
        for i in 0..count {
            for j in (i + 1)..count {
                let (a, b) = self.enemies.split_at_mut(j);
                let ea = &mut a[i];
                let eb = &mut b[0];
                let diff = vec3(ea.pos.x - eb.pos.x, 0.0, ea.pos.z - eb.pos.z);
                let distance = diff.length();
                let minimum = ea.kind.radius() + eb.kind.radius();
                if distance < minimum {
                    let direction = if distance > 0.001 {
                        diff / distance
                    } else if (i + j) % 2 == 0 {
                        vec3(1.0, 0.0, 0.0)
                    } else {
                        vec3(0.0, 0.0, 1.0)
                    };
                    let overlap = minimum - distance;
                    let push = direction * overlap * 0.65;
                    ea.pos += push;
                    eb.pos -= push;
                }
            }
        }

        // Move projectiles
        for p in self.projectiles.iter_mut() {
            if p.life <= 0.0 {
                continue;
            }
            p.life -= dt;
            p.pos += p.vel * dt;
        }

        // Wall collision (immutable borrows only — safe to iterate)
        let mut wall_hits: Vec<usize> = Vec::new();
        for (i, p) in self.projectiles.iter().enumerate() {
            if p.life > 0.0 && projectile_hits_world(p.pos, &self.pillars, &self.crates) {
                wall_hits.push(i);
            }
        }
        for i in wall_hits {
            self.projectiles[i].life = 0.0;
        }

        // Player collision + floor kill
        for p in self.projectiles.iter_mut() {
            if p.life > 0.0 {
                let distance = vec3(p.pos.x - target.x, 0.0, p.pos.z - target.z).length();
                if distance < PLAYER_R + 0.4 {
                    damage_taken += p.damage;
                    p.life = 0.0;
                }
            }
            if p.pos.y < 0.1 {
                p.life = 0.0;
            }
        }

        self.projectiles.retain(|p| p.life > 0.0);

        damage_taken
    }
}
