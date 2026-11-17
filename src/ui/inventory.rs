use macroquad::prelude::*;

use crate::systems::inventory::{set_loadout, toggle_attachment};
use crate::systems::save::Save;
use crate::ui::theme::*;
use crate::weapons::attachments::{AttachmentDef, ATTACHMENTS, SLOT_NAMES};
use crate::weapons::weapon::{WeaponCategory, WEAPONS};
use crate::State;

fn stat_effects_string(a: &AttachmentDef) -> String {
    let mut parts: Vec<String> = Vec::new();

    let dmg_pct = (a.damage_mult - 1.0) * 100.0;
    if dmg_pct.abs() >= 0.5 {
        parts.push(format!("DMG {:+.0}%", dmg_pct));
    }
    let acc_pct = (1.0 - a.spread_mult) * 100.0;
    if acc_pct.abs() >= 0.5 {
        parts.push(format!("ACC {:+.0}%", acc_pct));
    }
    let ctrl_pct = (1.0 - a.recoil_mult) * 100.0;
    if ctrl_pct.abs() >= 0.5 {
        parts.push(format!("CTRL {:+.0}%", ctrl_pct));
    }
    let rld_pct = (1.0 - a.reload_mult) * 100.0;
    if rld_pct.abs() >= 0.5 {
        parts.push(format!("RLD {:+.0}%", rld_pct));
    }
    if a.mag_add != 0 {
        parts.push(format!("MAG {:+}", a.mag_add));
    }
    if a.zoom > 1.01 {
        parts.push(format!("ZOOM {:.1}x", a.zoom));
    }

    if parts.is_empty() {
        "no stat change".to_string()
    } else {
        parts.join("   ")
    }
}

pub fn draw_inventory(
    save: &mut Save,
    sel_w: &mut usize,
    sel_slot: &mut usize,
    sel_att: &mut usize,
) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    let margin = S_8;

    draw_rectangle(0.0, 0.0, sw, sh, BG);

    // ===== Header =====
    text("INVENTORY", margin, 96.0, TEXT_H1, FG);
    text("Loadout & attachments", margin, 120.0, TEXT_MICRO, FG3);
    text_right(
        &format!("{} COINS", save.coins),
        sw - margin,
        96.0,
        TEXT_BODY,
        ACCENT,
    );
    hairline(margin, 136.0, sw - margin * 2.0);

    // ===== Weapon strip — grouped by category =====
    let wx = margin;
    let wy = 168.0;
    let ww = 115.0;
    let wh = 54.0;
    let gap = 6.0;

    // PRIMARY row
    text("PRIMARY", wx, wy - 6.0, TEXT_MICRO, FG3);
    let mut col = 0;
    let mut prim_positions: Vec<(usize, f32, f32)> = Vec::new();
    for i in 0..9 {
        if WeaponCategory::of(i) != WeaponCategory::Primary {
            continue;
        }
        let x = wx + col as f32 * (ww + gap);
        let owned = save.owned_weapons[i];
        let selected = save.loadout_primary == i;
        let focused = *sel_w == i;

        draw_rectangle(x, wy, ww, wh, if selected { SURFACE_HI } else { SURFACE });
        draw_rectangle_lines(x, wy, ww, wh, 1.0, if selected { ACCENT } else { BORDER });
        if focused {
            draw_rectangle(x, wy, 2.0, wh, ACCENT);
        }

        let c = if owned { WEAPONS[i].color } else { FG3 };
        text(WEAPONS[i].name, x + 6.0, wy + 22.0, TEXT_SMALL, c);
        text(
            if selected {
                "PRIMARY"
            } else if owned {
                "OWNED"
            } else {
                "LOCKED"
            },
            x + 6.0,
            wy + 42.0,
            9.0,
            if selected { ACCENT } else { FG3 },
        );
        prim_positions.push((i, x, wy));
        col += 1;
    }

    // SECONDARY row
    let wy2 = wy + wh + 14.0;
    text("SECONDARY", wx, wy2 - 6.0, TEXT_MICRO, FG3);
    col = 0;
    let mut sec_positions: Vec<(usize, f32, f32)> = Vec::new();
    for i in 0..9 {
        if WeaponCategory::of(i) != WeaponCategory::Secondary {
            continue;
        }
        let x = wx + col as f32 * (ww + gap);
        let owned = save.owned_weapons[i];
        let selected = save.loadout_secondary == i;
        let focused = *sel_w == i;

        draw_rectangle(x, wy2, ww, wh, if selected { SURFACE_HI } else { SURFACE });
        draw_rectangle_lines(x, wy2, ww, wh, 1.0, if selected { ACCENT } else { BORDER });
        if focused {
            draw_rectangle(x, wy2, 2.0, wh, ACCENT);
        }

        let c = if owned { WEAPONS[i].color } else { FG3 };
        text(WEAPONS[i].name, x + 6.0, wy2 + 22.0, TEXT_SMALL, c);
        text(
            if selected {
                "SECONDARY"
            } else if owned {
                "OWNED"
            } else {
                "LOCKED"
            },
            x + 6.0,
            wy2 + 42.0,
            9.0,
            if selected { ACCENT } else { FG3 },
        );
        sec_positions.push((i, x, wy2));
        col += 1;
    }

    // MELEE row
    let wy3 = wy2 + wh + 14.0;
    text("MELEE", wx, wy3 - 6.0, TEXT_MICRO, FG3);
    {
        let i = 1;
        let x = wx;
        draw_rectangle(x, wy3, ww, wh, SURFACE);
        draw_rectangle_lines(x, wy3, ww, wh, 1.0, BORDER);
        text(WEAPONS[i].name, x + 6.0, wy3 + 22.0, TEXT_SMALL, FG);
        text("ALWAYS", x + 6.0, wy3 + 42.0, 9.0, FG3);
    }

    // ===== Attachment slots (left) =====
    let slots_x = margin;
    let slots_y = wy3 + wh + 34.0;
    let slots_w = 400.0;
    let slot_h = 46.0;
    let slot_gap = 4.0;

    section_header(slots_x, slots_y - 8.0, slots_w, "SLOTS");

    let w_owned = save.owned_weapons[*sel_w];
    let w_is_melee = WEAPONS[*sel_w].is_melee;

    if is_key_pressed(KeyCode::Down) {
        *sel_slot = (*sel_slot + 1) % 5;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel_slot = (*sel_slot + 4) % 5;
    }

    for slot in 0..5 {
        let y = slots_y + S_4 + slot as f32 * (slot_h + slot_gap);
        let selected = *sel_slot == slot;

        if selected {
            draw_rectangle(slots_x, y, slots_w, slot_h, SURFACE);
            draw_rectangle(slots_x, y, 2.0, slot_h, ACCENT);
        }

        text(SLOT_NAMES[slot], slots_x + S_4, y + 20.0, TEXT_MICRO, FG3);

        let cur = save.weapon_attachments[*sel_w][slot];
        let content = if !w_owned {
            "Locked".to_string()
        } else if w_is_melee {
            "N/A".to_string()
        } else if cur >= 0 && (cur as usize) < ATTACHMENTS.len() {
            ATTACHMENTS[cur as usize].name.to_string()
        } else {
            "-".to_string()
        };
        let content_color = if cur >= 0 && w_owned && !w_is_melee {
            FG
        } else {
            FG3
        };
        text(&content, slots_x + S_4, y + 38.0, TEXT_BODY, content_color);
    }

    // ===== Available attachments (right, filtered by compatibility) =====
    let avail_x = margin + slots_w + S_7;
    let avail_y = slots_y;
    let avail_w = sw - avail_x - margin;

    section_header(avail_x, avail_y - 8.0, avail_w, "AVAILABLE");

    let mut slot_atts: Vec<usize> = Vec::new();
    for (ai, a) in ATTACHMENTS.iter().enumerate() {
        if a.slot != *sel_slot {
            continue;
        }
        if !crate::weapons::weapon::is_attachment_compatible(*sel_w, ai) {
            continue;
        }
        slot_atts.push(ai);
    }

    if slot_atts.is_empty() {
        text(
            if w_is_melee {
                "Melee weapons have no attachments."
            } else if !w_owned {
                "Buy this weapon to customize it."
            } else {
                "This weapon does not support attachments in this slot."
            },
            avail_x + S_4,
            avail_y + S_6,
            TEXT_SMALL,
            FG3,
        );
    }

    let row_h = 58.0;
    let mx = mouse_position().0;
    let my = mouse_position().1;
    let mut clicked_att: Option<usize> = None;

    for (row_idx, &ai) in slot_atts.iter().enumerate() {
        let a = &ATTACHMENTS[ai];
        let y = avail_y + S_4 + row_idx as f32 * (row_h + 4.0);
        let owned = save.owned_attachments[ai];
        let attached = save.weapon_attachments[*sel_w][*sel_slot] == ai as i32;
        let selected = *sel_att == ai;
        let hovered = mx >= avail_x && mx <= avail_x + avail_w && my >= y && my <= y + row_h;

        if selected || attached || hovered {
            draw_rectangle(avail_x, y, avail_w, row_h, SURFACE);
        }
        if selected {
            draw_rectangle(avail_x, y, 2.0, row_h, ACCENT);
        }

        let name_color = if owned { FG } else { FG2 };
        text(a.name, avail_x + S_4, y + 18.0, TEXT_BODY, name_color);
        text(a.desc, avail_x + S_4, y + 34.0, TEXT_MICRO, FG3);
        let effects = stat_effects_string(a);
        text(&effects, avail_x + S_4, y + 52.0, TEXT_MICRO, FG2);

        if attached {
            text_right(
                "ATTACHED",
                avail_x + avail_w - S_4,
                y + 20.0,
                TEXT_MICRO,
                ACCENT,
            );
            text_right(
                "ENTER: DETACH",
                avail_x + avail_w - S_4,
                y + 38.0,
                TEXT_MICRO,
                FG3,
            );
        } else if owned {
            text_right("OWNED", avail_x + avail_w - S_4, y + 20.0, TEXT_MICRO, FG3);
            text_right(
                "ENTER: ATTACH",
                avail_x + avail_w - S_4,
                y + 38.0,
                TEXT_MICRO,
                FG3,
            );
        } else {
            let affordable = save.coins >= a.price;
            let price_color = if affordable { ACCENT } else { FG3 };
            text_right(
                &format!("{} c", a.price),
                avail_x + avail_w - S_4,
                y + 20.0,
                TEXT_BODY,
                price_color,
            );
            if affordable {
                text_right(
                    "ENTER: BUY",
                    avail_x + avail_w - S_4,
                    y + 38.0,
                    TEXT_MICRO,
                    ACCENT,
                );
            } else {
                text_right(
                    "NEED COINS",
                    avail_x + avail_w - S_4,
                    y + 38.0,
                    TEXT_MICRO,
                    FG3,
                );
            }
        }

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            clicked_att = Some(ai);
        }
    }

    // ===== Navigation input =====
    if is_key_pressed(KeyCode::Left) {
        *sel_w = (*sel_w + 8) % 9;
    }
    if is_key_pressed(KeyCode::Right) {
        *sel_w = (*sel_w + 1) % 9;
    }

    // Enter = set loadout if looking at a weapon, else attachment action
    if is_key_pressed(KeyCode::Enter) {
        let cat = WeaponCategory::of(*sel_w);
        match cat {
            WeaponCategory::Primary | WeaponCategory::Secondary => {
                let _ = set_loadout(save, *sel_w);
            }
            WeaponCategory::Melee => {}
        }
    }

    // Number-key attachment select (1-5 within current slot's list)
    let keys = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
    ];
    for (k_idx, k) in keys.iter().enumerate() {
        if is_key_pressed(*k) && k_idx < slot_atts.len() {
            *sel_att = slot_atts[k_idx];
        }
    }

    // Attachment action (ENTER on attachment = buy/attach/detach)
    let mut do_att_action = clicked_att.is_some();
    if let Some(ai) = clicked_att {
        *sel_att = ai;
    }
    if is_key_pressed(KeyCode::Enter) && slot_atts.contains(sel_att) {
        do_att_action = true;
    }

    if do_att_action {
        let can_edit = save.owned_weapons[*sel_w] && !WEAPONS[*sel_w].is_melee;
        if can_edit {
            let ai = *sel_att;
            if ai < ATTACHMENTS.len() && ATTACHMENTS[ai].slot == *sel_slot {
                if save.owned_attachments[ai] {
                    let _ = toggle_attachment(save, *sel_w, *sel_slot, ai);
                } else {
                    let price = ATTACHMENTS[ai].price;
                    if save.coins >= price {
                        save.coins -= price;
                        save.owned_attachments[ai] = true;
                        save.weapon_attachments[*sel_w][*sel_slot] = ai as i32;
                    }
                }
            }
        }
    }

    // Footer
    let footer_y = sh - margin;
    hairline(margin, footer_y - 30.0, sw - margin * 2.0);
    text(
        "ARROWS: NAVIGATE   |   ENTER: SET LOADOUT / BUY / ATTACH   |   1-5: ATTACHMENT   |   ESC: BACK",
        margin, footer_y, TEXT_MICRO, FG3,
    );

    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Menu);
    }
    None
}
