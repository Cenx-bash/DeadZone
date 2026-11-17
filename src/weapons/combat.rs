use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::*;

use crate::game::Game;
use crate::maps::arena::{HALF, PILLAR_R};
use crate::systems::audio;
use crate::systems::save::Save;
use crate::weapons::attachments::compute_mods;
use crate::weapons::weapon::WEAPONS;

pub struct Tracer {
    pub a: Vec3,
    pub b: Vec3,
    pub life: f32,
}
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
    pub size: f32,
}
pub struct Shell {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
}
pub struct FloatingText {
    pub pos: Vec3,
    pub text: String,
    pub life: f32,
    pub color: Color,
}
pub struct Grenade {
    pub pos: Vec3,
    pub vel: Vec3,
    pub fuse: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MineKind {
    Landmine,
    Claymore,
}

pub struct Mine {
    pub pos: Vec3,
    pub kind: MineKind,
    pub arm_t: f32,
}

fn ray_aabb(origin: Vec3, dir: Vec3, min: Vec3, max: Vec3) -> Option<f32> {
    let inv_x = if dir.x.abs() > 1e-6 { 1.0 / dir.x } else { 1e6 };
    let inv_y = if dir.y.abs() > 1e-6 { 1.0 / dir.y } else { 1e6 };
    let inv_z = if dir.z.abs() > 1e-6 { 1.0 / dir.z } else { 1e6 };

    let t1x = (min.x - origin.x) * inv_x;
    let t2x = (max.x - origin.x) * inv_x;
    let tminx = t1x.min(t2x);
    let tmaxx = t1x.max(t2x);

    let t1y = (min.y - origin.y) * inv_y;
    let t2y = (max.y - origin.y) * inv_y;
    let tminy = t1y.min(t2y);
    let tmaxy = t1y.max(t2y);

    let t1z = (min.z - origin.z) * inv_z;
    let t2z = (max.z - origin.z) * inv_z;
    let tminz = t1z.min(t2z);
    let tmaxz = t1z.max(t2z);

    let t_enter = tminx.max(tminy).max(tminz);
    let t_exit = tmaxx.min(tmaxy).min(tmaxz);

    if t_enter <= t_exit && t_exit > 0.0 && t_enter > 0.0 {
        Some(t_enter)
    } else {
        None
    }
}

struct GunMeshBuilder {
    verts: Vec<Vertex>,
    indices: Vec<u16>,
    f: Vec3,
    r: Vec3,
    u: Vec3,
    base: Vec3,
}

impl GunMeshBuilder {
    fn new(base: Vec3, f: Vec3, r: Vec3, u: Vec3) -> Self {
        Self {
            verts: Vec::with_capacity(256),
            indices: Vec::with_capacity(512),
            f,
            r,
            u,
            base,
        }
    }

    fn add(&mut self, fwd: f32, right: f32, up: f32, hf: f32, hr: f32, hu: f32, color: Color) {
        let center = self.base + self.f * fwd + self.r * right + self.u * up;
        let ff = self.f * hf;
        let rr = self.r * hr;
        let uu = self.u * hu;

        let c8: [u8; 4] = [
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        ];

        let c0 = center - ff - rr - uu;
        let c1 = center - ff + rr - uu;
        let c2 = center - ff + rr + uu;
        let c3 = center - ff - rr + uu;
        let c4 = center + ff - rr - uu;
        let c5 = center + ff + rr - uu;
        let c6 = center + ff + rr + uu;
        let c7 = center + ff - rr + uu;

        let b = self.verts.len() as u16;
        for pos in [c0, c1, c2, c3, c4, c5, c6, c7] {
            self.verts.push(Vertex {
                position: pos,
                uv: Vec2::ZERO,
                color: c8,
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
            });
        }

        let tris: [[u16; 3]; 24] = [
            [b + 0, b + 1, b + 2],
            [b + 0, b + 2, b + 3],
            [b + 4, b + 6, b + 5],
            [b + 4, b + 7, b + 6],
            [b + 3, b + 2, b + 6],
            [b + 3, b + 6, b + 7],
            [b + 0, b + 4, b + 5],
            [b + 0, b + 5, b + 1],
            [b + 1, b + 5, b + 6],
            [b + 1, b + 6, b + 2],
            [b + 0, b + 3, b + 7],
            [b + 0, b + 7, b + 4],
            [b + 2, b + 1, b + 0],
            [b + 3, b + 2, b + 0],
            [b + 5, b + 6, b + 4],
            [b + 6, b + 7, b + 4],
            [b + 6, b + 2, b + 3],
            [b + 7, b + 6, b + 3],
            [b + 5, b + 4, b + 0],
            [b + 1, b + 5, b + 0],
            [b + 6, b + 5, b + 1],
            [b + 2, b + 6, b + 1],
            [b + 7, b + 3, b + 0],
            [b + 4, b + 7, b + 0],
        ];
        for t in tris {
            self.indices.push(t[0]);
            self.indices.push(t[1]);
            self.indices.push(t[2]);
        }
    }

    fn finish(self) -> Mesh {
        Mesh {
            vertices: self.verts,
            indices: self.indices,
            texture: None,
        }
    }
}

impl Game {
    pub(crate) fn burst(&mut self, at: Vec3, n: usize, color: Color, speed: f32) {
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

    pub(crate) fn reap_dead(&mut self) {
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].hp <= 0.0 {
                let e = self.enemies.swap_remove(i);
                let reward = e.kind.score();
                self.score += reward;
                self.player.coins += reward / 2;
                self.kills += 1;
                let center = vec3(e.pos.x, e.kind.height() * 0.4, e.pos.z);
                self.burst(center, 16, e.kind.color(), 5.0);
                let roll = rand::gen_range(0.0, 1.0);
                if roll < 0.12 && self.player.armor < 50.0 {
                    self.player.armor = (self.player.armor + 10.0).min(50.0);
                    audio::sfx_pickup();
                } else if roll < 0.22 {
                    self.player.reserve += 15;
                    audio::sfx_pickup();
                } else if roll < 0.25 {
                    self.player.grenade_count += 1;
                    audio::sfx_pickup();
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

    pub(crate) fn shoot(&mut self) {
        let w = WEAPONS[self.player.equipped];
        self.player.cooldown = 1.0 / w.fire_rate;
        self.player.recoil =
            (self.player.recoil + w.recoil * 0.12 * self.player.recoil_mult).min(0.9);
        self.player.muzzle_flash = 0.06;
        if !w.is_melee {
            self.player.ammo = self.player.ammo.saturating_sub(1);
        }

        match self.player.equipped {
            1 => audio::sfx_knife(),
            3 | 5 => audio::sfx_shotgun(),
            _ => audio::sfx_shot(),
        }

        let (f, r, u) = self.player.basis();
        let o = self.player.eye();
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
        let spread = w.spread * self.player.spread_mult * if self.player.ads { 0.4 } else { 1.0 };
        let dmg_base =
            w.damage * self.player.damage_mult * (1.0 + self.player.ability_lv[5] as f32 * 0.10);

        let mut hits: Vec<(usize, f32, Vec3, bool)> = Vec::new();
        let mut crate_hits: Vec<(usize, f32)> = Vec::new();

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

            let mut hit_crate: Option<usize> = None;
            if !w.is_melee {
                for (ci, c) in self.crates.iter().enumerate() {
                    if c.hp <= 0.0 {
                        continue;
                    }
                    let bmin = vec3(c.x - c.hw, 0.0, c.z - c.hd);
                    let bmax = vec3(c.x + c.hw, c.h, c.z + c.hd);
                    if let Some(t) = ray_aabb(o, dir, bmin, bmax) {
                        if t > 0.0 && t < best_t {
                            best_t = t;
                            best_idx = None;
                            hit_crate = Some(ci);
                        }
                    }
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
                            hit_crate = None;
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
                            hit_crate = None;
                        }
                    }
                }
                if dir.y < -1e-4 {
                    let t = -o.y / dir.y;
                    if t > 0.0 && t < best_t {
                        best_t = t;
                        best_idx = None;
                        hit_crate = None;
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
                    dmg *= 2.0 + self.player.ability_lv[8] as f32 * 0.20;
                }
                hits.push((i, dmg, end, headshot));
            } else if let Some(ci) = hit_crate {
                crate_hits.push((ci, dmg_base));
            } else if w.is_melee {
                self.burst(o + f * 1.5, 4, Color::new(0.9, 0.9, 0.9, 1.0), 2.0);
            }
        }

        let mut destroyed_crates: Vec<usize> = Vec::new();
        for (ci, dmg) in crate_hits {
            if ci >= self.crates.len() {
                continue;
            }
            let was_alive = self.crates[ci].hp > 0.0;
            self.crates[ci].hp -= dmg;
            if was_alive && self.crates[ci].hp <= 0.0 {
                destroyed_crates.push(ci);
            }
        }
        for ci in destroyed_crates {
            let c = &self.crates[ci];
            let center = vec3(c.x, c.h * 0.5, c.z);
            self.burst(center, 20, Color::new(0.6, 0.5, 0.3, 1.0), 5.0);
            audio::sfx_hit();
            let roll = rand::gen_range(0.0, 1.0);
            if roll < 0.4 {
                self.player.reserve += 20;
                audio::sfx_pickup();
            } else if roll < 0.7 {
                self.player.armor = (self.player.armor + 10.0).min(50.0);
                audio::sfx_pickup();
            } else if roll < 0.9 {
                self.player.coins += 15;
                audio::sfx_pickup();
            } else {
                self.player.grenade_count += 1;
                audio::sfx_pickup();
            }
        }

        let mut any_hit = false;
        let mut any_headshot = false;
        for (i, dmg, point, hs) in hits {
            if i >= self.enemies.len() {
                continue;
            }
            any_hit = true;
            if hs {
                any_headshot = true;
            }
            self.enemies[i].hp -= dmg;
            self.enemies[i].flash = 0.08;
            self.player.hit_mark = 0.12;
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
            if self.player.ability_lv[9] > 0 && !w.is_melee {
                let radius = 1.2 + self.player.ability_lv[9] as f32 * 0.5;
                let aoe = dmg * 0.25 * self.player.ability_lv[9] as f32;
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
        if any_hit {
            if any_headshot {
                audio::sfx_headshot();
            } else {
                audio::sfx_hit();
            }
        }

        self.reap_dead();
    }

    pub(crate) fn throw_grenade(&mut self) {
        if self.player.grenade_count == 0 || self.player.grenade_cd > 0.0 {
            return;
        }
        let (f, _, _) = self.player.basis();
        let eye = self.player.eye();
        let vel = f * 16.0 + vec3(0.0, 4.0, 0.0);
        self.grenades.push(Grenade {
            pos: eye + f * 0.4,
            vel,
            fuse: 2.5,
        });
        self.player.grenade_count -= 1;
        self.player.grenade_cd = 0.4;
        audio::sfx_knife();
    }

    pub(crate) fn update_grenades(&mut self, dt: f32) {
        self.player.grenade_cd = (self.player.grenade_cd - dt).max(0.0);
        for g in self.grenades.iter_mut() {
            g.fuse -= dt;
            g.vel.y -= crate::game::player::GRAVITY * dt;
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
            let pd = vec3(self.player.pos.x - pos.x, 0.0, self.player.pos.z - pos.z).length();
            if pd < radius * 0.7 {
                let dmg = 30.0 * (1.0 - pd / (radius * 0.7));
                let absorb = dmg.min(self.player.armor);
                self.player.armor -= absorb;
                self.player.health -= dmg - absorb;
                self.player.hurt_flash = 0.5;
            }
            self.burst(pos, 50, Color::new(1.0, 0.6, 0.2, 1.0), 10.0);
            self.burst(pos, 20, Color::new(1.0, 0.9, 0.3, 1.0), 6.0);
            self.player.shake = 1.0;
            audio::sfx_explosion();
        }
        self.reap_dead();
    }

    pub(crate) fn update_deploy(&mut self, dt: f32) {
        self.player.deploy_cd = (self.player.deploy_cd - dt).max(0.0);
        if self.player.deploy_cd > 0.0 {
            return;
        }

        if is_key_pressed(KeyCode::M) && self.player.landmine_count > 0 {
            self.place_mine(MineKind::Landmine);
            self.player.landmine_count -= 1;
            self.player.deploy_cd = 0.4;
        } else if is_key_pressed(KeyCode::N) && self.player.claymore_count > 0 {
            self.place_mine(MineKind::Claymore);
            self.player.claymore_count -= 1;
            self.player.deploy_cd = 0.4;
        } else if is_key_pressed(KeyCode::X) && self.player.nuke_count > 0 {
            self.detonate_nuke();
            self.player.nuke_count -= 1;
            self.player.deploy_cd = 1.0;
        }
    }

    fn place_mine(&mut self, kind: MineKind) {
        let (f, _, _) = self.player.basis();
        let p = self.player.pos + f * 1.2;
        self.mines.push(Mine {
            pos: vec3(p.x, 0.05, p.z),
            kind,
            arm_t: 0.8,
        });
        audio::sfx_click();
    }

    pub(crate) fn update_mines(&mut self, dt: f32) {
        for m in self.mines.iter_mut() {
            m.arm_t = (m.arm_t - dt).max(0.0);
        }

        let mut det_idx: Vec<usize> = Vec::new();
        for (i, m) in self.mines.iter().enumerate() {
            if m.arm_t > 0.0 {
                continue;
            }
            for e in &self.enemies {
                let d = vec3(m.pos.x - e.pos.x, 0.0, m.pos.z - e.pos.z).length();
                let trigger = match m.kind {
                    MineKind::Landmine => d < 1.5,
                    MineKind::Claymore => d < 4.0,
                };
                if trigger {
                    det_idx.push(i);
                    break;
                }
            }
        }

        let mut dets: Vec<(Vec3, MineKind)> = Vec::new();
        for i in det_idx.into_iter().rev() {
            let m = self.mines.swap_remove(i);
            dets.push((m.pos, m.kind));
        }

        for (pos, kind) in dets {
            let (radius, dmg) = match kind {
                MineKind::Landmine => (4.0, 120.0),
                MineKind::Claymore => (6.0, 200.0),
            };
            for e in self.enemies.iter_mut() {
                let d = vec3(e.pos.x - pos.x, 0.0, e.pos.z - pos.z).length();
                if d < radius {
                    let fall = 1.0 - (d / radius);
                    e.hp -= dmg * fall;
                }
            }
            self.burst(pos, 40, Color::new(1.0, 0.5, 0.2, 1.0), 8.0);
            self.burst(pos, 20, Color::new(1.0, 0.9, 0.4, 1.0), 5.0);
            audio::sfx_explosion();
            self.player.shake = (self.player.shake + 0.5).min(1.0);
        }

        self.reap_dead();
    }

    fn detonate_nuke(&mut self) {
        self.nuke_flash = 1.0;
        self.player.shake = 1.5;
        audio::sfx_explosion();

        let count = self.enemies.len() as u32;
        self.score += count * 10;
        self.player.coins += count * 5;
        self.kills += count;

        let center = self.player.pos;
        for _ in 0..300 {
            let v = vec3(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(0.0, 2.0),
                rand::gen_range(-1.0, 1.0),
            ) * 20.0;
            self.particles.push(Particle {
                pos: vec3(center.x, 0.5, center.z),
                vel: v,
                life: 1.5,
                max_life: 1.5,
                color: Color::new(1.0, 0.8, 0.3, 1.0),
                size: 0.15,
            });
        }

        self.enemies.clear();
    }

    pub(crate) fn switch_weapon(&mut self, idx: usize, save: &Save) {
        if idx >= 9 || !save.owned_weapons[idx] {
            return;
        }

        let cat = crate::weapons::weapon::WeaponCategory::of(idx);
        let allowed = match cat {
            crate::weapons::weapon::WeaponCategory::Primary => idx == save.loadout_primary,
            crate::weapons::weapon::WeaponCategory::Secondary => idx == save.loadout_secondary,
            crate::weapons::weapon::WeaponCategory::Melee => true,
        };
        if !allowed || idx == self.player.equipped {
            return;
        }

        self.player.ammo_per_weapon[self.player.equipped] = self.player.ammo;
        self.player.reserve_per_weapon[self.player.equipped] = self.player.reserve;

        self.player.equipped = idx;
        self.player.reload_t = 0.0;
        self.player.cooldown = 0.35;

        let (recoil_mult, spread_mult, reload_mult, damage_mult, mag_bonus, zoom_mult) =
            compute_mods(save, idx);
        self.player.recoil_mult = recoil_mult;
        self.player.spread_mult = spread_mult;
        self.player.reload_mult = reload_mult;
        self.player.damage_mult = damage_mult;
        self.player.mag_bonus = mag_bonus;
        self.player.zoom_mult = zoom_mult;

        let w = WEAPONS[idx];
        if w.is_melee {
            self.player.ammo = 0;
            self.player.reserve = 0;
        } else {
            self.player.ammo = self.player.ammo_per_weapon[idx];
            self.player.reserve = self.player.reserve_per_weapon[idx];
        }
    }

    pub(crate) fn cycle_loadout_weapon(&mut self, save: &Save, dir: i32) {
        let order: [usize; 3] = [save.loadout_primary, save.loadout_secondary, 1];
        let pos = order
            .iter()
            .position(|&x| x == self.player.equipped)
            .unwrap_or(0);
        for step in 1..=3 {
            let next = ((pos as i32 + dir * step).rem_euclid(3)) as usize;
            let target = order[next];
            if save.owned_weapons[target] && target != self.player.equipped {
                self.switch_weapon(target, save);
                return;
            }
        }
    }

    pub(crate) fn update_firing(&mut self, dt: f32) {
        let w = WEAPONS[self.player.equipped];
        self.player.cooldown -= dt;

        if self.player.reload_t > 0.0 {
            self.player.reload_t -= dt;
            if self.player.reload_t <= 0.0 {
                self.player.reload_t = 0.0;
                let need = self.player.current_mag() - self.player.ammo;
                let take = need.min(self.player.reserve);
                self.player.ammo += take;
                self.player.reserve -= take;
            }
        }

        let want_fire = if w.auto {
            is_mouse_button_down(MouseButton::Left)
        } else {
            is_mouse_button_pressed(MouseButton::Left)
        };

        if self.player.reload_t <= 0.0 {
            if w.is_melee {
                if want_fire && self.player.cooldown <= 0.0 {
                    self.shoot();
                }
            } else {
                if is_key_pressed(KeyCode::R)
                    && self.player.ammo < self.player.current_mag()
                    && self.player.reserve > 0
                {
                    self.player.reload_t = self.player.current_reload();
                    audio::sfx_reload();
                } else if want_fire && self.player.cooldown <= 0.0 {
                    if self.player.ammo > 0 {
                        self.shoot();
                    } else if self.player.reserve > 0 {
                        self.player.reload_t = self.player.current_reload();
                        audio::sfx_reload();
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::G) {
            self.throw_grenade();
        }
    }

    pub(crate) fn update_fx(&mut self, dt: f32) {
        self.player.muzzle_flash = (self.player.muzzle_flash - dt).max(0.0);
        self.player.recoil = (self.player.recoil - dt * 4.0).max(0.0);
        self.player.hit_mark = (self.player.hit_mark - dt).max(0.0);
        self.player.hurt_flash = (self.player.hurt_flash - dt).max(0.0);
        self.player.shake = (self.player.shake - dt * 3.0).max(0.0);
        self.nuke_flash = (self.nuke_flash - dt * 0.8).max(0.0);

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

        self.player.ammo_per_weapon[self.player.equipped] = self.player.ammo;
        self.player.reserve_per_weapon[self.player.equipped] = self.player.reserve;
    }

    pub(crate) fn draw_weapon(&self, eye: Vec3, f: Vec3, r: Vec3, u: Vec3) {
        let w = WEAPONS[self.player.equipped];

        let reload_offset = if self.player.reload_t > 0.0 {
            let p = 1.0 - (self.player.reload_t / self.player.current_reload().max(0.01));
            (p * std::f32::consts::PI).sin() * 0.35
        } else {
            0.0
        };
        let ads = if self.player.ads { 1.0 } else { 0.0 };
        let recoil = self.player.recoil;
        let sway = self.player.mouse_sway;

        let base = eye
            + f * (0.40 + ads * 0.05 - recoil * 0.06 - reload_offset * 0.4)
            + r * (0.24 - ads * 0.24 + sway.x * 0.04)
            - u * (0.24 - ads * 0.10 - reload_offset * 0.3 - sway.y * 0.04);

        let metal = Color::new(0.22, 0.22, 0.24, 1.0);
        let metal_hi = Color::new(0.34, 0.34, 0.38, 1.0);
        let dark = Color::new(0.10, 0.10, 0.12, 1.0);
        let grip = Color::new(0.14, 0.14, 0.16, 1.0);
        let wood = Color::new(0.36, 0.22, 0.12, 1.0);
        let accent = w.color;
        let skin = Color::new(0.85, 0.66, 0.52, 1.0);

        let mut b = GunMeshBuilder::new(base, f, r, u);

        let muzzle_fwd: f32 = match self.player.equipped {
            0 => {
                b.add(0.16, 0.00, 0.025, 0.15, 0.028, 0.040, metal);
                b.add(0.10, 0.00, -0.025, 0.10, 0.025, 0.020, dark);
                b.add(0.32, 0.00, 0.025, 0.03, 0.018, 0.022, metal_hi);
                b.add(0.00, 0.00, -0.10, 0.035, 0.028, 0.075, grip);
                b.add(0.09, 0.00, -0.08, 0.030, 0.020, 0.005, dark);
                b.add(0.04, 0.00, 0.070, 0.010, 0.022, 0.006, dark);
                b.add(0.29, 0.00, 0.070, 0.008, 0.018, 0.010, dark);
                b.add(0.02, 0.00, -0.14, 0.035, 0.030, 0.035, skin);
                0.35
            }
            1 => {
                b.add(0.05, 0.00, -0.05, 0.055, 0.018, 0.022, dark);
                b.add(0.11, 0.00, -0.05, 0.008, 0.038, 0.038, metal);
                b.add(0.19, 0.00, -0.05, 0.070, 0.006, 0.020, metal_hi);
                b.add(0.27, 0.00, -0.05, 0.010, 0.005, 0.010, metal_hi);
                b.add(0.03, 0.00, -0.06, 0.040, 0.028, 0.030, skin);
                0.28
            }
            2 => {
                b.add(0.22, 0.00, 0.020, 0.18, 0.038, 0.045, metal);
                b.add(0.18, 0.00, 0.075, 0.14, 0.012, 0.010, dark);
                b.add(0.42, 0.00, 0.025, 0.05, 0.022, 0.022, dark);
                b.add(0.48, 0.00, 0.025, 0.02, 0.026, 0.026, metal_hi);
                b.add(0.06, 0.00, -0.09, 0.032, 0.026, 0.075, grip);
                b.add(0.16, 0.00, -0.14, 0.026, 0.022, 0.075, dark);
                b.add(-0.02, 0.00, 0.020, 0.030, 0.020, 0.030, dark);
                b.add(0.40, 0.00, 0.070, 0.008, 0.014, 0.008, dark);
                b.add(0.08, 0.00, 0.070, 0.010, 0.020, 0.008, dark);
                b.add(0.05, 0.00, -0.10, 0.030, 0.026, 0.030, skin);
                b.add(0.34, 0.00, -0.04, 0.030, 0.026, 0.030, skin);
                0.50
            }
            3 => {
                b.add(0.20, 0.00, 0.020, 0.14, 0.045, 0.045, metal);
                b.add(0.38, 0.00, -0.020, 0.07, 0.055, 0.032, wood);
                b.add(0.58, 0.00, 0.045, 0.10, 0.020, 0.020, metal_hi);
                b.add(0.58, 0.00, 0.005, 0.10, 0.020, 0.020, metal_hi);
                b.add(0.68, 0.00, 0.045, 0.008, 0.024, 0.024, dark);
                b.add(0.68, 0.00, 0.005, 0.008, 0.024, 0.024, dark);
                b.add(-0.05, 0.00, 0.020, 0.10, 0.040, 0.045, wood);
                b.add(0.05, 0.00, -0.075, 0.030, 0.028, 0.055, grip);
                b.add(0.03, 0.00, -0.08, 0.032, 0.026, 0.032, skin);
                b.add(0.38, 0.00, -0.03, 0.032, 0.028, 0.030, skin);
                0.70
            }
            4 => {
                b.add(0.18, 0.00, 0.020, 0.16, 0.032, 0.045, metal);
                b.add(0.38, 0.00, 0.005, 0.10, 0.036, 0.032, dark);
                b.add(0.55, 0.00, 0.015, 0.10, 0.013, 0.013, metal_hi);
                b.add(0.65, 0.00, 0.015, 0.015, 0.020, 0.020, dark);
                b.add(0.04, 0.00, -0.085, 0.030, 0.026, 0.065, grip);
                b.add(0.10, 0.00, -0.10, 0.026, 0.020, 0.065, dark);
                b.add(-0.06, 0.00, 0.020, 0.09, 0.032, 0.038, grip);
                b.add(0.14, 0.00, 0.085, 0.05, 0.020, 0.020, dark);
                b.add(0.16, 0.00, 0.130, 0.04, 0.018, 0.018, dark);
                b.add(0.20, 0.00, 0.130, 0.004, 0.014, 0.014, accent);
                b.add(0.03, 0.00, -0.09, 0.030, 0.026, 0.030, skin);
                b.add(0.36, 0.00, -0.03, 0.030, 0.026, 0.030, skin);
                0.67
            }
            5 => {
                b.add(0.20, 0.00, 0.020, 0.20, 0.028, 0.042, metal);
                b.add(0.55, 0.00, 0.020, 0.15, 0.012, 0.012, metal_hi);
                b.add(0.70, 0.00, 0.020, 0.020, 0.018, 0.018, dark);
                b.add(0.16, 0.00, 0.115, 0.10, 0.028, 0.028, dark);
                b.add(0.26, 0.00, 0.115, 0.006, 0.026, 0.026, accent);
                b.add(
                    0.06,
                    0.00,
                    0.115,
                    0.006,
                    0.022,
                    0.022,
                    Color::new(0.1, 0.2, 0.3, 1.0),
                );
                b.add(0.16, 0.00, 0.075, 0.010, 0.010, 0.020, dark);
                b.add(0.04, 0.00, -0.090, 0.030, 0.026, 0.070, grip);
                b.add(0.10, 0.00, -0.090, 0.022, 0.020, 0.045, dark);
                b.add(-0.10, 0.00, 0.020, 0.10, 0.028, 0.045, grip);
                b.add(0.60, 0.06, -0.04, 0.006, 0.004, 0.040, dark);
                b.add(0.60, -0.06, -0.04, 0.006, 0.004, 0.040, dark);
                b.add(0.03, 0.00, -0.09, 0.030, 0.026, 0.032, skin);
                b.add(0.36, 0.00, -0.02, 0.030, 0.026, 0.030, skin);
                0.72
            }
            6 => {
                b.add(0.22, 0.00, 0.020, 0.24, 0.048, 0.058, metal);
                b.add(0.44, 0.00, 0.010, 0.08, 0.045, 0.040, dark);
                b.add(0.58, 0.00, 0.020, 0.10, 0.016, 0.016, metal_hi);
                b.add(0.68, 0.00, 0.020, 0.02, 0.022, 0.022, dark);
                b.add(0.14, 0.09, -0.060, 0.10, 0.040, 0.100, dark);
                b.add(
                    0.22,
                    0.06,
                    -0.020,
                    0.010,
                    0.030,
                    0.030,
                    Color::new(0.5, 0.4, 0.2, 1.0),
                );
                b.add(0.02, 0.00, -0.100, 0.030, 0.028, 0.070, grip);
                b.add(0.20, 0.00, 0.110, 0.05, 0.012, 0.012, dark);
                b.add(-0.08, 0.00, 0.020, 0.09, 0.035, 0.050, grip);
                b.add(-0.02, 0.00, -0.080, 0.025, 0.024, 0.055, grip);
                b.add(0.01, 0.00, -0.10, 0.030, 0.026, 0.032, skin);
                b.add(0.44, 0.00, -0.02, 0.030, 0.026, 0.030, skin);
                0.70
            }
            7 => {
                b.add(0.14, 0.00, 0.020, 0.09, 0.026, 0.038, metal);
                b.add(0.05, 0.00, 0.020, 0.040, 0.040, 0.040, metal_hi);
                b.add(0.32, 0.00, 0.020, 0.15, 0.016, 0.020, metal);
                b.add(0.48, 0.00, 0.020, 0.015, 0.020, 0.024, dark);
                b.add(-0.01, 0.00, 0.055, 0.012, 0.014, 0.020, dark);
                b.add(0.00, 0.00, -0.080, 0.032, 0.026, 0.075, wood);
                b.add(0.08, 0.00, -0.060, 0.028, 0.020, 0.008, dark);
                b.add(0.40, 0.00, 0.052, 0.008, 0.014, 0.010, dark);
                b.add(0.00, 0.00, -0.10, 0.032, 0.026, 0.032, skin);
                0.50
            }
            8 => {
                b.add(0.20, 0.00, 0.020, 0.18, 0.030, 0.042, metal);
                b.add(0.40, 0.00, 0.005, 0.10, 0.034, 0.032, dark);
                b.add(0.58, 0.00, 0.015, 0.13, 0.012, 0.012, metal_hi);
                b.add(0.70, 0.00, 0.015, 0.015, 0.018, 0.018, dark);
                b.add(0.04, 0.00, -0.085, 0.030, 0.026, 0.065, grip);
                b.add(0.10, 0.00, -0.095, 0.024, 0.020, 0.055, dark);
                b.add(-0.06, 0.00, 0.020, 0.10, 0.030, 0.040, grip);
                b.add(0.16, 0.00, 0.085, 0.05, 0.016, 0.018, dark);
                b.add(0.18, 0.00, 0.120, 0.055, 0.020, 0.020, dark);
                b.add(0.23, 0.00, 0.120, 0.005, 0.016, 0.016, accent);
                b.add(0.03, 0.00, -0.09, 0.030, 0.026, 0.030, skin);
                b.add(0.38, 0.00, -0.03, 0.030, 0.026, 0.030, skin);
                0.72
            }
            _ => 0.5,
        };

        let mesh = b.finish();
        draw_mesh(&mesh);

        if self.player.muzzle_flash > 0.0 && !w.is_melee {
            let a = (self.player.muzzle_flash / 0.06).clamp(0.0, 1.0);
            let flash_pos = base + f * muzzle_fwd;
            draw_sphere(flash_pos, 0.10, None, Color::new(1.0, 0.85, 0.35, a));
            draw_sphere(flash_pos, 0.20, None, Color::new(1.0, 0.6, 0.2, a * 0.5));
            draw_sphere(flash_pos, 0.32, None, Color::new(1.0, 0.4, 0.1, a * 0.2));
        }
    }
}
