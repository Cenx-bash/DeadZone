pub mod camera;
pub mod movement;
pub mod player;
pub mod wave_manager;

use macroquad::prelude::*;

use crate::maps::arena::{maps, Crate, Pillar, HALF};
use crate::systems::audio;
use crate::systems::save::Save;
use crate::weapons::combat::{FloatingText, Grenade, Mine, MineKind, Particle, Shell, Tracer};
use crate::weapons::weapon::WEAPONS;
use crate::zombies::ai::Projectile;
use crate::zombies::enemy::Enemy;

use camera::{look_dir, rotate_around_axis};
use player::Player;
use wave_manager::WaveManager;

pub struct Game {
    pub player: Player,
    pub waves: WaveManager,
    pub kills: u32,
    pub score: u32,
    pub pillars: Vec<Pillar>,
    pub crates: Vec<Crate>,
    pub floor: Color,
    pub tile: Color,
    pub wall: Color,
    pub wall_trim: Color,
    pub accent: Color,
    pub fog: Color,
    pub map_name: &'static str,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub tracers: Vec<Tracer>,
    pub particles: Vec<Particle>,
    pub shells: Vec<Shell>,
    pub floaters: Vec<FloatingText>,
    pub grenades: Vec<Grenade>,
    pub mines: Vec<Mine>,
    pub nuke_flash: f32,
}

impl Game {
    pub fn new(save: &Save, map_idx: usize) -> Self {
        let mdefs = maps();
        let def = &mdefs[map_idx % mdefs.len()];
        let equipped = save.equipped_weapon.min(8);
        let player = Player::new(save, equipped);
        let waves = WaveManager::new();
        let _ = WEAPONS;
        Self {
            player,
            waves,
            kills: 0,
            score: 0,
            pillars: def.pillars.clone(),
            crates: def.crates.clone(),
            floor: def.floor,
            tile: def.tile,
            wall: def.wall,
            wall_trim: def.wall_trim,
            accent: def.accent,
            fog: def.fog,
            map_name: def.name,
            enemies: Vec::new(),
            projectiles: Vec::new(),
            tracers: Vec::new(),
            particles: Vec::new(),
            shells: Vec::new(),
            floaters: Vec::new(),
            grenades: Vec::new(),
            mines: Vec::new(),
            nuke_flash: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, look: Vec2, save: &Save, wheel_y: f32) {
        self.update_look(dt, look);
        self.update_weapon_switch(save, wheel_y);
        self.update_move(dt);
        self.update_jump(dt);
        self.update_firing(dt);
        self.update_deploy(dt);
        self.update_wave(dt);

        let dmg = self.update_enemies(dt);
        if dmg > 0.0 {
            let absorb = dmg.min(self.player.armor);
            self.player.armor -= absorb;
            self.player.health -= dmg - absorb;
            self.player.hurt_flash = 0.35;
            self.player.shake = (self.player.shake + 0.4).min(1.0);
            audio::sfx_hurt();
        }
        if self.player.ability_lv[4] > 0 && self.player.health > 0.0 {
            let rate = self.player.ability_lv[4] as f32;
            self.player.health = (self.player.health + rate * dt).min(self.player.max_health);
        }

        self.update_grenades(dt);
        self.update_mines(dt);
        self.update_fx(dt);
    }

    pub fn draw_world(&self) {
        let (f, r, u) = self.player.basis();
        let eye = self.player.eye();

        let base_fov = self.player.fov_deg.to_radians();
        let kick = self.player.sprint_kick * 0.10;
        let fovy = if self.player.ads {
            base_fov / self.player.zoom_mult.max(1.0)
        } else {
            base_fov + kick
        };

        let roll = self.player.strafe_lean * 0.02;
        let up = if roll.abs() > 0.0001 {
            rotate_around_axis(Vec3::Y, f, roll)
        } else {
            Vec3::Y
        };

        let shake_off = vec3(
            rand::gen_range(-1.0, 1.0) * self.player.shake * 0.05,
            rand::gen_range(-1.0, 1.0) * self.player.shake * 0.05,
            0.0,
        );
        set_camera(&Camera3D {
            position: eye + shake_off,
            target: eye + shake_off + f,
            up,
            fovy,
            ..Default::default()
        });

        let tiles = 8;
        let tile_size = (HALF * 2.0) / tiles as f32;
        for i in 0..tiles {
            for j in 0..tiles {
                let x = -HALF + i as f32 * tile_size + tile_size * 0.5;
                let z = -HALF + j as f32 * tile_size + tile_size * 0.5;
                let color = if (i + j) % 2 == 0 {
                    self.floor
                } else {
                    self.tile
                };
                draw_plane(vec3(x, -0.01, z), vec2(tile_size, tile_size), None, color);
            }
        }

        let len = HALF * 2.0 + 1.0;
        let walls = [
            (vec3(0.0, 2.0, -HALF - 0.5), vec3(len, 4.0, 1.0)),
            (vec3(0.0, 2.0, HALF + 0.5), vec3(len, 4.0, 1.0)),
            (vec3(-HALF - 0.5, 2.0, 0.0), vec3(1.0, 4.0, len)),
            (vec3(HALF + 0.5, 2.0, 0.0), vec3(1.0, 4.0, len)),
        ];
        for (pos, size) in walls {
            draw_cube(pos, size, None, self.wall);
            draw_cube_wires(pos, size, self.wall_trim);
        }

        for p in &self.pillars {
            let c = vec3(p.x, 2.0, p.z);
            draw_cube(c, vec3(1.6, 4.0, 1.6), None, self.wall);
            draw_cube_wires(c, vec3(1.6, 4.0, 1.6), self.accent);
            draw_cube(
                vec3(p.x, 4.05, p.z),
                vec3(1.75, 0.1, 1.75),
                None,
                self.accent,
            );
        }

        // Mines
        for m in &self.mines {
            match m.kind {
                MineKind::Landmine => {
                    draw_cube(
                        vec3(m.pos.x, 0.1, m.pos.z),
                        vec3(0.35, 0.08, 0.35),
                        None,
                        Color::new(0.85, 0.65, 0.15, 1.0),
                    );
                    if m.arm_t <= 0.0 {
                        draw_cube_wires(
                            vec3(m.pos.x, 0.1, m.pos.z),
                            vec3(0.35, 0.08, 0.35),
                            Color::new(1.0, 0.85, 0.25, 1.0),
                        );
                    }
                }
                MineKind::Claymore => {
                    draw_cube(
                        vec3(m.pos.x, 0.25, m.pos.z),
                        vec3(0.35, 0.5, 0.1),
                        None,
                        Color::new(0.7, 0.25, 0.25, 1.0),
                    );
                    draw_cube_wires(
                        vec3(m.pos.x, 0.25, m.pos.z),
                        vec3(0.35, 0.5, 0.1),
                        Color::new(1.0, 0.5, 0.3, 1.0),
                    );
                }
            }
        }

        for c in &self.crates {
            if c.hp <= 0.0 {
                continue;
            }
            let center = vec3(c.x, c.h * 0.5, c.z);
            let size = vec3(c.hw * 2.0, c.h, c.hd * 2.0);
            draw_cube(center, size, None, self.wall);
            draw_cube_wires(center, size, self.accent);
            draw_cube(
                vec3(c.x, c.h + 0.03, c.z),
                vec3(c.hw * 2.0 + 0.08, 0.06, c.hd * 2.0 + 0.08),
                None,
                self.accent,
            );
        }

        for e in &self.enemies {
            let h = e.kind.height();
            let w = e.kind.radius() * 1.6;
            let bob = e.phase.sin().abs() * 0.10;
            let col = if e.flash > 0.0 { WHITE } else { e.kind.color() };
            let crawler = matches!(e.kind, crate::zombies::enemy::ZombieKind::Crawler);
            let body_y = if crawler { h * 0.3 } else { h * 0.35 };
            let body_h = if crawler { h * 0.5 } else { h * 0.7 };
            let body = vec3(e.pos.x, body_y + bob, e.pos.z);
            draw_cube(body, vec3(w, body_h, w * 0.8), None, col);
            draw_cube_wires(body, vec3(w, body_h, w * 0.8), BLACK);

            if !crawler {
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
        let _ = look_dir;
    }
}
