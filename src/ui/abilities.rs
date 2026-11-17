use macroquad::prelude::*;

use crate::systems::abilities::{ability_cost, ABILITIES};
use crate::systems::save::Save;
use crate::ui::theme::*;
use crate::State;

pub fn draw_abilities(save: &mut Save, sel: &mut usize) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    let margin = S_8;

    draw_rectangle(0.0, 0.0, sw, sh, BG);

    // ===== Header =====
    text("ABILITIES", margin, 96.0, TEXT_H1, FG);
    text("Permanent upgrades", margin, 120.0, TEXT_MICRO, FG3);
    text_right(
        &format!("{} COINS", save.coins),
        sw - margin,
        96.0,
        TEXT_BODY,
        ACCENT,
    );
    hairline(margin, 136.0, sw - margin * 2.0);

    // ===== Nav =====
    if is_key_pressed(KeyCode::Right) {
        *sel = (*sel + 1).min(9);
    }
    if is_key_pressed(KeyCode::Left) {
        *sel = (*sel).saturating_sub(1);
    }
    if is_key_pressed(KeyCode::Down) {
        if *sel + 2 <= 9 {
            *sel += 2;
        }
    }
    if is_key_pressed(KeyCode::Up) {
        if *sel >= 2 {
            *sel -= 2;
        }
    }

    // ===== Grid =====
    let cols = 2;
    let gap = S_4;
    let grid_y = 176.0;
    let card_w = (sw - margin * 2.0 - gap) / cols as f32;
    let card_h = 110.0;

    let mx = mouse_position().0;
    let my = mouse_position().1;
    let mut clicked: Option<usize> = None;

    for i in 0..10 {
        let a = &ABILITIES[i];
        let col = i % cols;
        let row = i / cols;
        let x = margin + col as f32 * (card_w + gap);
        let y = grid_y + row as f32 * (card_h + gap);
        let lv = save.ability_levels[i];
        let maxed = lv >= a.max_level;
        let cost = if maxed { 0 } else { ability_cost(i, lv) };
        let affordable = save.coins >= cost;

        let hovered = mx >= x && mx <= x + card_w && my >= y && my <= y + card_h;
        let selected = *sel == i;

        card(x, y, card_w, card_h, selected, hovered);

        let pad = S_4;

        // Name + description
        text(a.name, x + pad, y + 28.0, TEXT_H3, FG);
        text(a.desc, x + pad, y + 48.0, TEXT_MICRO, FG3);

        // Level pips
        let pips_y = y + 66.0;
        for p in 0..a.max_level {
            let px = x + pad + p as f32 * 14.0;
            if p < lv {
                draw_rectangle(px, pips_y, 10.0, 3.0, ACCENT);
            } else {
                draw_rectangle(px, pips_y, 10.0, 3.0, FG4);
            }
        }

        // Price / status (bottom)
        let by = y + card_h - pad;
        if maxed {
            text("MAX", x + pad, by, TEXT_SMALL, SUCCESS);
        } else {
            let color = if affordable { ACCENT } else { FG3 };
            text(&format!("{} c", cost), x + pad, by, TEXT_SMALL, color);
            text_right(
                &format!("LV {}/{}", lv, a.max_level),
                x + card_w - pad,
                by,
                TEXT_MICRO,
                FG3,
            );
        }

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            clicked = Some(i);
        }
    }

    // Handle click purchase
    let mut purchased = false;
    if let Some(i) = clicked {
        *sel = i;
        let a = &ABILITIES[i];
        let lv = save.ability_levels[i];
        if lv < a.max_level {
            let cost = ability_cost(i, lv);
            if save.coins >= cost {
                save.coins -= cost;
                save.ability_levels[i] += 1;
                purchased = true;
            }
        }
    }

    // Enter purchase
    if !purchased && is_key_pressed(KeyCode::Enter) {
        let a = &ABILITIES[*sel];
        let lv = save.ability_levels[*sel];
        if lv < a.max_level {
            let cost = ability_cost(*sel, lv);
            if save.coins >= cost {
                save.coins -= cost;
                save.ability_levels[*sel] += 1;
            }
        }
    }

    // Footer
    let footer_y = sh - margin;
    hairline(margin, footer_y - 30.0, sw - margin * 2.0);
    text(
        "ARROWS NAVIGATE   |   CLICK OR ENTER TO PURCHASE   |   ESC BACK",
        margin,
        footer_y,
        TEXT_MICRO,
        FG3,
    );

    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Menu);
    }
    None
}
