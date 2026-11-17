use macroquad::prelude::*;

use crate::systems::save::Save;
use crate::ui::theme::*;
use crate::weapons::weapon::WEAPONS;
use crate::State;

pub fn draw_shop(save: &mut Save, sel: &mut usize) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    let margin = S_8;

    draw_rectangle(0.0, 0.0, sw, sh, BG);

    // ===== Header =====
    text("ARMORY", margin, 96.0, TEXT_H1, FG);
    text("Weapons", margin, 120.0, TEXT_MICRO, FG3);
    text_right(
        &format!("{} COINS", save.coins),
        sw - margin,
        96.0,
        TEXT_BODY,
        ACCENT,
    );
    hairline(margin, 136.0, sw - margin * 2.0);

    // ===== Grid nav =====
    if is_key_pressed(KeyCode::Right) {
        *sel = (*sel + 1).min(8);
    }
    if is_key_pressed(KeyCode::Left) {
        *sel = (*sel).saturating_sub(1);
    }
    if is_key_pressed(KeyCode::Down) {
        if *sel + 3 <= 8 {
            *sel += 3;
        }
    }
    if is_key_pressed(KeyCode::Up) {
        if *sel >= 3 {
            *sel -= 3;
        }
    }

    // ===== Grid =====
    let cols = 3;
    let gap = S_4;
    let cards_y = 176.0;
    let card_w = (sw - margin * 2.0 - gap * (cols - 1) as f32) / cols as f32;
    let card_h = 140.0;

    let mx = mouse_position().0;
    let my = mouse_position().1;
    let mut clicked: Option<usize> = None;

    for i in 0..9 {
        let col = i % cols;
        let row = i / cols;
        let x = margin + col as f32 * (card_w + gap);
        let y = cards_y + row as f32 * (card_h + gap);
        let w = &WEAPONS[i];

        let owned = save.owned_weapons[i];
        let equipped = save.equipped_weapon == i;
        let hovered = mx >= x && mx <= x + card_w && my >= y && my <= y + card_h;
        let selected = *sel == i;

        card(x, y, card_w, card_h, selected, hovered);

        let pad = S_4;

        // Name + rarity
        text(
            w.name,
            x + pad,
            y + 30.0,
            TEXT_H3,
            if owned || equipped { FG } else { FG2 },
        );
        text(
            w.rarity.name(),
            x + pad,
            y + 46.0,
            TEXT_MICRO,
            w.rarity.color(),
        );

        // Divider
        hairline(x + pad, y + 58.0, card_w - pad * 2.0);

        // Stats
        let stat_w = (card_w - pad * 2.0) / 3.0;
        let sx = x + pad;
        let sy = y + 86.0;

        text(&format!("{:.0}", w.damage), sx, sy, TEXT_H3, FG);
        text("DMG", sx, sy + 14.0, TEXT_MICRO, FG3);

        text(&format!("{:.1}", w.fire_rate), sx + stat_w, sy, TEXT_H3, FG);
        text("RATE", sx + stat_w, sy + 14.0, TEXT_MICRO, FG3);

        if w.is_melee {
            text("-", sx + stat_w * 2.0, sy, TEXT_H3, FG3);
        } else {
            text(
                &format!("{}", w.mag_size),
                sx + stat_w * 2.0,
                sy,
                TEXT_H3,
                FG,
            );
        }
        text("MAG", sx + stat_w * 2.0, sy + 14.0, TEXT_MICRO, FG3);

        // Action line
        let by = y + card_h - pad;
        if equipped {
            text("EQUIPPED", x + pad, by, TEXT_MICRO, ACCENT);
        } else if owned {
            text("OWNED", x + pad, by, TEXT_MICRO, FG3);
            text_right("EQUIP >", x + card_w - pad, by, TEXT_MICRO, FG2);
        } else if save.coins >= w.price {
            text(&format!("{} c", w.price), x + pad, by, TEXT_SMALL, ACCENT);
            text_right("BUY >", x + card_w - pad, by, TEXT_MICRO, ACCENT);
        } else {
            text(&format!("{} c", w.price), x + pad, by, TEXT_SMALL, FG3);
        }

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            clicked = Some(i);
        }
    }

    if let Some(i) = clicked {
        *sel = i;
        let w = &WEAPONS[i];
        if save.owned_weapons[i] {
            save.equipped_weapon = i;
        } else if save.coins >= w.price {
            save.coins -= w.price;
            save.owned_weapons[i] = true;
            save.equipped_weapon = i;
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        let w = &WEAPONS[*sel];
        if save.owned_weapons[*sel] {
            save.equipped_weapon = *sel;
        } else if save.coins >= w.price {
            save.coins -= w.price;
            save.owned_weapons[*sel] = true;
            save.equipped_weapon = *sel;
        }
    }

    // Footer
    let footer_y = sh - margin;
    hairline(margin, footer_y - 30.0, sw - margin * 2.0);
    text(
        "ARROWS NAVIGATE   |   CLICK OR ENTER TO BUY/EQUIP   |   ESC BACK",
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
