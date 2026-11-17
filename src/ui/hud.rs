pub use super::theme::*;
use macroquad::prelude::*;

use crate::game::player::STAMINA_MAX;
use crate::game::Game;
use crate::weapons::weapon::{WeaponCategory, WEAPONS};

impl Game {
    pub fn draw_hud(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let margin = S_7;

        // Damage flash
        if self.player.hurt_flash > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                sw,
                sh,
                Color::new(0.9, 0.1, 0.1, self.player.hurt_flash * 0.35),
            );
        }

        // Nuke flash
        if self.nuke_flash > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                sw,
                sh,
                Color::new(1.0, 1.0, 1.0, self.nuke_flash * 0.85),
            );
        }

        self.draw_crosshair();

        // ============================================================
        // TOP CENTER — Wave + enemies (single tight line)
        // ============================================================
        let remaining = self.enemies.len() + self.waves.to_spawn as usize;
        let wave_line = if remaining > 0 {
            format!("WAVE {}   {} REMAINING", self.waves.wave, remaining)
        } else {
            format!("WAVE {}   CLEAR", self.waves.wave)
        };
        let ww = measure_text(&wave_line, None, 15, 1.0).width;
        let wave_bg_w = ww + 40.0;
        let wave_bg_h = 26.0;
        let wave_bg_x = sw / 2.0 - wave_bg_w / 2.0;
        let wave_bg_y = 18.0;
        draw_rectangle(
            wave_bg_x,
            wave_bg_y,
            wave_bg_w,
            wave_bg_h,
            Color::new(0.05, 0.06, 0.09, 0.7),
        );
        draw_rectangle_lines(
            wave_bg_x,
            wave_bg_y,
            wave_bg_w,
            wave_bg_h,
            1.0,
            Color::new(0.20, 0.22, 0.28, 1.0),
        );
        draw_text(&wave_line, sw / 2.0 - ww / 2.0, wave_bg_y + 18.0, 15.0, FG);

        // ============================================================
        // TOP LEFT — Coins + Kills (stacked tight)
        // ============================================================
        text("COINS", margin, 26.0, TEXT_MICRO, FG3);
        draw_text(
            &format!("{}", self.player.coins),
            margin,
            52.0,
            28.0,
            ACCENT,
        );

        text("KILLS", margin, 74.0, TEXT_MICRO, FG3);
        draw_text(&format!("{}", self.kills), margin, 96.0, 22.0, FG2);

        // ============================================================
        // BOTTOM LEFT — Vitals (compact 3-bar stack)
        // ============================================================
        let bar_w = 210.0;
        let bar_h = 6.0;
        let lx = margin;
        let vy = sh - margin - 78.0;

        // HP
        text("HP", lx, vy + 8.0, TEXT_MICRO, FG3);
        draw_text(
            &format!("{:.0}", self.player.health),
            lx + 22.0,
            vy + 8.0,
            TEXT_SMALL,
            if self.player.health < 30.0 {
                DANGER
            } else {
                FG
            },
        );
        let hp_pct = (self.player.health / self.player.max_health).max(0.0);
        draw_rectangle(
            lx,
            vy + 12.0,
            bar_w,
            bar_h,
            Color::new(0.05, 0.05, 0.07, 0.85),
        );
        draw_rectangle(
            lx,
            vy + 12.0,
            bar_w * hp_pct,
            bar_h,
            if self.player.health < 30.0 {
                DANGER
            } else {
                Color::new(0.55, 0.85, 0.55, 1.0)
            },
        );

        // Armor
        text("AP", lx, vy + 34.0, TEXT_MICRO, FG3);
        draw_text(
            &format!("{:.0}", self.player.armor),
            lx + 22.0,
            vy + 34.0,
            TEXT_SMALL,
            FG2,
        );
        let ar_pct = (self.player.armor / 50.0).clamp(0.0, 1.0);
        draw_rectangle(
            lx,
            vy + 38.0,
            bar_w,
            bar_h,
            Color::new(0.05, 0.05, 0.07, 0.85),
        );
        draw_rectangle(
            lx,
            vy + 38.0,
            bar_w * ar_pct,
            bar_h,
            Color::new(0.40, 0.65, 1.00, 1.0),
        );

        // Stamina
        text("STA", lx, vy + 60.0, TEXT_MICRO, FG3);
        draw_text(
            &format!("{:.0}", self.player.stamina),
            lx + 26.0,
            vy + 60.0,
            TEXT_SMALL,
            FG2,
        );
        let st_pct = self.player.stamina / STAMINA_MAX;
        draw_rectangle(
            lx,
            vy + 64.0,
            bar_w,
            bar_h,
            Color::new(0.05, 0.05, 0.07, 0.85),
        );
        draw_rectangle(
            lx,
            vy + 64.0,
            bar_w * st_pct,
            bar_h,
            Color::new(0.95, 0.75, 0.30, 1.0),
        );

        // ============================================================
        // BOTTOM RIGHT — Weapon panel: icon beside ammo
        // ============================================================
        let w = WEAPONS[self.player.equipped];
        let cat = WeaponCategory::of(self.player.equipped);
        let cat_label = match cat {
            WeaponCategory::Primary => "PRIMARY",
            WeaponCategory::Secondary => "SECONDARY",
            WeaponCategory::Melee => "MELEE",
        };

        // Pre-measure the ammo content so the panel can size itself
        let ammo_size = 38.0;
        let ammo_str = format!("{}", self.player.ammo);
        let res_str = format!("/{}", self.player.reserve);
        let melee_str = "MELEE";
        let (am_w, content_w) = if w.is_melee {
            let mw = measure_text(melee_str, None, 30, 1.0).width;
            (0.0, mw)
        } else {
            let a = measure_text(&ammo_str, None, ammo_size as u16, 1.0).width;
            let r = measure_text(&res_str, None, TEXT_BODY as u16, 1.0).width;
            (a, a + 6.0 + r)
        };

        let name_w = measure_text(w.name, None, 18, 1.0).width;
        let cat_w = measure_text(cat_label, None, 11, 1.0).width;

        let pad = 12.0;
        let gap = 16.0;
        let icon_w = 96.0;
        let icon_h = 40.0;
        let panel_h = 84.0;
        let body_w = pad + icon_w + gap + content_w + pad;
        let header_w = pad + name_w + 20.0 + cat_w + pad;
        let panel_w = body_w.max(header_w);
        let px = sw - margin - panel_w;
        let py = sh - margin - panel_h;

        // Panel background + border + colored accent edge
        draw_rectangle(px, py, panel_w, panel_h, Color::new(0.05, 0.06, 0.09, 0.78));
        draw_rectangle_lines(
            px,
            py,
            panel_w,
            panel_h,
            1.0,
            Color::new(0.20, 0.22, 0.28, 1.0),
        );
        draw_rectangle(px + panel_w - 3.0, py, 3.0, panel_h, w.color);

        // Header: name (left) + category (right)
        draw_text(w.name, px + pad, py + 20.0, 18.0, w.color);
        draw_text(
            cat_label,
            px + panel_w - pad - 3.0 - cat_w,
            py + 18.0,
            11.0,
            FG3,
        );

        // Divider
        draw_line(
            px + pad,
            py + 28.0,
            px + panel_w - pad - 3.0,
            py + 28.0,
            1.0,
            Color::new(0.20, 0.22, 0.28, 1.0),
        );

        // Weapon icon (left of ammo)
        let icon_x = px + pad;
        let icon_y = py + 32.0 + (panel_h - 32.0 - 8.0 - icon_h) / 2.0;
        draw_weapon_icon(icon_x, icon_y, icon_w, icon_h, cat, w.color);

        // Ammo block (right-aligned, baseline near bottom)
        let content_right = px + panel_w - pad - 3.0;
        let base_y = py + panel_h - 16.0;
        if w.is_melee {
            draw_text(
                melee_str,
                content_right - content_w,
                base_y + 2.0,
                30.0,
                FG2,
            );
        } else {
            let start_x = content_right - content_w;
            let ammo_color = if self.player.ammo == 0 {
                DANGER
            } else if self.player.ammo <= 3 {
                Color::new(1.0, 0.75, 0.35, 1.0)
            } else {
                FG
            };
            draw_text(&ammo_str, start_x, base_y, ammo_size, ammo_color);
            draw_text(&res_str, start_x + am_w + 6.0, base_y, TEXT_BODY, FG3);

            // Reload bar
            if self.player.reload_t > 0.0 {
                let p = 1.0
                    - (self.player.reload_t / self.player.current_reload().max(0.01))
                        .clamp(0.0, 1.0);
                let by = py + panel_h - 8.0;
                draw_rectangle(
                    start_x,
                    by,
                    content_w,
                    2.0,
                    Color::new(0.15, 0.15, 0.20, 1.0),
                );
                draw_rectangle(start_x, by, content_w * p, 2.0, ACCENT);
            }
        }

        // ============================================================
        // BOTTOM CENTER — Consumables strip (thin, low-profile)
        // ============================================================
        self.draw_consumables_strip(sw, sh);

        // ============================================================
        // Hit marker
        // ============================================================
        if self.player.hit_mark > 0.0 {
            let a = (self.player.hit_mark / 0.12).clamp(0.0, 1.0);
            let c = Color::new(DANGER.r, DANGER.g, DANGER.b, a);
            let (cx, cy) = (sw / 2.0, sh / 2.0);
            let d = 5.0;
            let e = 12.0;
            draw_line(cx - e, cy - e, cx - d, cy - d, 1.5, c);
            draw_line(cx + d, cy + d, cx + e, cy + e, 1.5, c);
            draw_line(cx + e, cy - e, cx + d, cy - d, 1.5, c);
            draw_line(cx - d, cy + d, cx - e, cy + e, 1.5, c);
        }

        // Wave incoming banner
        if self.waves.to_spawn == 0 && self.enemies.is_empty() {
            let m = format!("WAVE {} INCOMING", self.waves.wave + 1);
            let d = measure_text(&m, None, 32, 1.0);
            draw_text(&m, sw / 2.0 - d.width / 2.0, sh / 2.0 - 60.0, 32.0, ACCENT);
        }
    }

    fn draw_consumables_strip(&self, sw: f32, sh: f32) {
        let items: [(&str, &str, u32, Color); 4] = [
            (
                "G",
                "GRENADE",
                self.player.grenade_count,
                Color::new(0.5, 0.9, 0.5, 1.0),
            ),
            (
                "M",
                "MINE",
                self.player.landmine_count,
                Color::new(0.9, 0.75, 0.3, 1.0),
            ),
            (
                "N",
                "CLAYMORE",
                self.player.claymore_count,
                Color::new(0.9, 0.4, 0.4, 1.0),
            ),
            (
                "X",
                "NUKE",
                self.player.nuke_count,
                Color::new(1.0, 0.6, 0.3, 1.0),
            ),
        ];

        let cell_w = 96.0;
        let cell_h = 30.0;
        let gap = 4.0;
        let total_w = 4.0 * cell_w + 3.0 * gap;
        let start_x = sw / 2.0 - total_w / 2.0;
        let y = sh - 42.0;

        for (i, (key, name, count, color)) in items.iter().enumerate() {
            let x = start_x + i as f32 * (cell_w + gap);

            let bg_alpha = if *count > 0 { 0.88 } else { 0.45 };
            draw_rectangle(x, y, cell_w, cell_h, Color::new(0.06, 0.07, 0.10, bg_alpha));
            let border_color = if *count > 0 {
                *color
            } else {
                Color::new(0.20, 0.22, 0.28, 1.0)
            };
            draw_rectangle_lines(x, y, cell_w, cell_h, 1.0, border_color);

            // Key cap
            draw_rectangle(
                x + 4.0,
                y + 4.0,
                20.0,
                22.0,
                Color::new(0.12, 0.13, 0.17, 1.0),
            );
            draw_rectangle_lines(
                x + 4.0,
                y + 4.0,
                20.0,
                22.0,
                1.0,
                Color::new(0.25, 0.27, 0.33, 1.0),
            );
            let key_w = measure_text(key, None, 13, 1.0).width;
            draw_text(
                key,
                x + 4.0 + 10.0 - key_w / 2.0,
                y + 20.0,
                13.0,
                if *count > 0 { FG } else { FG3 },
            );

            // Name
            draw_text(name, x + 28.0, y + 12.0, 9.0, FG3);

            // Count
            let count_str = format!("x{}", count);
            let cw = measure_text(&count_str, None, 15, 1.0).width;
            draw_text(
                &count_str,
                x + cell_w - 6.0 - cw,
                y + 22.0,
                15.0,
                if *count > 0 { *color } else { FG3 },
            );
        }

        // Shop hint above the strip
        let hint = "B  ·  FIELD SHOP";
        let hint_w = measure_text(hint, None, 10, 1.0).width;
        draw_text(
            hint,
            sw / 2.0 - hint_w / 2.0,
            y - 6.0,
            10.0,
            Color::new(0.45, 0.48, 0.55, 1.0),
        );
    }

    fn draw_crosshair(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let (cx, cy) = (sw / 2.0, sh / 2.0);

        if self.player.ads {
            draw_circle_lines(cx, cy, 2.0, 1.0, FG2);
            draw_circle_lines(cx, cy, 40.0, 1.0, FG4);
            return;
        }

        let spread = WEAPONS[self.player.equipped].spread * self.player.spread_mult;
        let gap = 5.0 + spread * 180.0;
        let len = 6.0;

        draw_rectangle(cx - 0.5, cy - gap - len, 1.0, len, FG);
        draw_rectangle(cx - 0.5, cy + gap, 1.0, len, FG);
        draw_rectangle(cx - gap - len, cy - 0.5, len, 1.0, FG);
        draw_rectangle(cx + gap, cy - 0.5, len, 1.0, FG);
        draw_rectangle(cx - 1.0, cy - 1.0, 2.0, 2.0, FG);
    }
}

/// Procedural weapon silhouette, drawn in a 100x40 design box scaled to (w, h).
fn draw_weapon_icon(x: f32, y: f32, w: f32, h: f32, cat: WeaponCategory, color: Color) {
    let u = w / 100.0;
    let v = h / 40.0;
    let shadow = Color::new(0.0, 0.0, 0.0, 0.55);
    let detail = Color::new(color.r * 0.6, color.g * 0.6, color.b * 0.6, color.a);

    // Two passes: drop shadow, then the real thing
    for (ox, oy, main, det) in [
        (1.5_f32, 1.5_f32, shadow, shadow),
        (0.0_f32, 0.0_f32, color, detail),
    ] {
        let r = |rx: f32, ry: f32, rw: f32, rh: f32, c: Color| {
            draw_rectangle(x + ox + rx * u, y + oy + ry * v, rw * u, rh * v, c);
        };
        match cat {
            WeaponCategory::Primary => {
                r(0.0, 12.0, 22.0, 12.0, det); // stock
                r(20.0, 10.0, 36.0, 12.0, main); // receiver
                r(34.0, 4.0, 16.0, 6.0, det); // optic
                r(56.0, 12.0, 26.0, 8.0, main); // handguard
                r(82.0, 14.0, 18.0, 4.0, main); // barrel
                r(90.0, 10.0, 3.0, 4.0, det); // front sight
                r(38.0, 22.0, 9.0, 15.0, det); // magazine
                r(26.0, 22.0, 7.0, 13.0, main); // grip
            }
            WeaponCategory::Secondary => {
                r(22.0, 8.0, 54.0, 11.0, main); // slide
                r(76.0, 11.0, 8.0, 4.0, det); // barrel tip
                r(22.0, 19.0, 40.0, 5.0, det); // frame
                r(22.0, 19.0, 13.0, 19.0, main); // grip
                r(40.0, 24.0, 12.0, 2.0, det); // trigger guard
                r(26.0, 5.0, 3.0, 3.0, det); // rear sight
            }
            WeaponCategory::Melee => {
                r(6.0, 17.0, 28.0, 7.0, det); // handle
                r(34.0, 11.0, 4.0, 19.0, main); // guard
                draw_triangle(
                    // blade
                    vec2(x + ox + 38.0 * u, y + oy + 12.0 * v),
                    vec2(x + ox + 38.0 * u, y + oy + 28.0 * v),
                    vec2(x + ox + 96.0 * u, y + oy + 22.0 * v),
                    main,
                );
            }
        }
    }
}
