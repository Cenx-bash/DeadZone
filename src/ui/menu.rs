use macroquad::prelude::*;

use crate::maps::arena::maps;
use crate::systems::save::Save;
use crate::ui::theme::*;
use crate::weapons::weapon::WEAPONS;

pub enum MenuCmd {
    Play,
    Shop,
    Inventory,
    Abilities,
    Settings,
    CycleMap,
    Quit,
}

pub fn draw_loading(load_t: f32) {
    let sw = screen_width();
    let sh = screen_height();
    let p = (load_t / 1.6).min(1.0);

    draw_rectangle(0.0, 0.0, sw, sh, BG);

    let cy = sh * 0.46;
    text_center("DEADZONE", sw / 2.0, cy, TEXT_DISPLAY, FG);
    text_center("ZOMBIE SURVIVAL", sw / 2.0, cy + 26.0, TEXT_MICRO, FG3);

    let bw = 240.0;
    let bx = sw / 2.0 - bw / 2.0;
    let by = sh * 0.65;
    draw_rectangle(bx, by, bw, 1.0, BORDER);
    draw_rectangle(bx, by, bw * p, 1.0, ACCENT);

    text_right(
        &format!("{:.0}%", p * 100.0),
        sw / 2.0 + bw / 2.0,
        by - 12.0,
        TEXT_MICRO,
        FG3,
    );
}

pub fn draw_menu(save: &Save, sel: &mut usize, map_idx: &mut usize) -> Option<MenuCmd> {
    let sw = screen_width();
    let sh = screen_height();
    let margin = S_8;

    draw_rectangle(0.0, 0.0, sw, sh, BG);

    text("DEADZONE", margin, 96.0, TEXT_DISPLAY, FG);
    text("ZOMBIE SURVIVAL", margin, 120.0, TEXT_MICRO, FG3);
    hairline(margin, 140.0, 280.0);

    let menu_x = margin;
    let menu_y = 200.0;
    let menu_w = 320.0;
    let row_h = 44.0;
    let row_gap = 2.0;

    const N: usize = 7;
    if is_key_pressed(KeyCode::Down) {
        *sel = (*sel + 1) % N;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel = (*sel + N - 1) % N;
    }

    let maps_list = maps();
    let m = &maps_list[*map_idx % maps_list.len()];
    let map_name = format!("{} · {}", m.name, m.tagline);

    let items: [(&str, String); N] = [
        ("Play", String::new()),
        ("Armory", String::new()),
        ("Inventory", String::new()),
        ("Abilities", String::new()),
        ("Settings", String::new()),
        ("Map", map_name),
        ("Quit", String::new()),
    ];

    let mx = mouse_position().0;
    let my = mouse_position().1;
    let mut cmd: Option<MenuCmd> = None;

    for (i, (label, value)) in items.iter().enumerate() {
        let y = menu_y + i as f32 * (row_h + row_gap);
        let focused = *sel == i;
        let hovered = mx >= menu_x && mx <= menu_x + menu_w && my >= y && my <= y + row_h;
        let active = focused || hovered;

        if active {
            draw_rectangle(menu_x, y, menu_w, row_h, SURFACE);
        }
        if focused {
            draw_rectangle(menu_x + S_4, y + row_h / 2.0 - 2.0, 4.0, 4.0, ACCENT);
        }

        text(
            label,
            menu_x + S_5,
            y + 28.0,
            TEXT_H3,
            if active { FG } else { FG2 },
        );

        if !value.is_empty() {
            text_right(
                value,
                menu_x + menu_w - S_4,
                y + 28.0,
                TEXT_SMALL,
                if active { ACCENT } else { FG3 },
            );
        }

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            *sel = i;
            cmd = Some(match i {
                0 => MenuCmd::Play,
                1 => MenuCmd::Shop,
                2 => MenuCmd::Inventory,
                3 => MenuCmd::Abilities,
                4 => MenuCmd::Settings,
                5 => MenuCmd::CycleMap,
                _ => MenuCmd::Quit,
            });
        }
    }

    let panel_w = 280.0;
    let panel_x = sw - margin - panel_w;
    let panel_y = 200.0;

    section_header(panel_x, panel_y, panel_w, "PROFILE");

    let mut row_y = panel_y + S_6;
    let prof_row_h = 28.0;
    stat_row(
        panel_x,
        row_y,
        panel_w,
        "COINS",
        &format!("{}", save.coins),
        FG,
    );
    row_y += prof_row_h;
    stat_row(
        panel_x,
        row_y,
        panel_w,
        "BEST WAVE",
        &format!("{}", save.best_wave),
        FG,
    );
    row_y += prof_row_h;
    stat_row(
        panel_x,
        row_y,
        panel_w,
        "KILLS",
        &format!("{}", save.total_kills),
        FG,
    );

    row_y += S_5 * 2.0;
    section_header(panel_x, row_y, panel_w, "EQUIPPED");

    row_y += S_6;
    let idx = save.equipped_weapon.min(8);
    let w = WEAPONS[idx];
    text(w.name, panel_x, row_y, TEXT_BODY, FG);
    text_right(
        w.rarity.name(),
        panel_x + panel_w,
        row_y,
        TEXT_MICRO,
        w.rarity.color(),
    );

    let footer_y = sh - margin;
    hairline(margin, footer_y - 30.0, sw - margin * 2.0);
    text(
        "W/S NAVIGATE   |   ENTER SELECT   |   M CYCLE MAP   |   Q QUIT",
        margin,
        footer_y,
        TEXT_MICRO,
        FG3,
    );
    text_right("v0.4", sw - margin, footer_y, TEXT_MICRO, FG4);

    if cmd.is_none() && is_key_pressed(KeyCode::Enter) {
        cmd = Some(match *sel {
            0 => MenuCmd::Play,
            1 => MenuCmd::Shop,
            2 => MenuCmd::Inventory,
            3 => MenuCmd::Abilities,
            4 => MenuCmd::Settings,
            5 => MenuCmd::CycleMap,
            _ => MenuCmd::Quit,
        });
    }
    if cmd.is_none() && is_key_pressed(KeyCode::M) {
        cmd = Some(MenuCmd::CycleMap);
    }
    if cmd.is_none() && is_key_pressed(KeyCode::Q) {
        cmd = Some(MenuCmd::Quit);
    }

    cmd
}
