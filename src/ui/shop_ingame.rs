use macroquad::prelude::*;

use crate::game::Game;
use crate::systems::audio;
use crate::systems::shop_items::{ItemKind, INGAME_SHOP};
use crate::ui::theme::*;

pub fn draw_field_shop(game: &mut Game, sel: &mut usize) {
    let sw = screen_width();
    let sh = screen_height();

    // Dark overlay
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.85));

    // Panel
    let pw = 760.0;
    let ph = 580.0;
    let px = sw / 2.0 - pw / 2.0;
    let py = sh / 2.0 - ph / 2.0;

    draw_rectangle(px, py, pw, ph, SURFACE);
    draw_rectangle_lines(px, py, pw, ph, 2.0, ACCENT);

    // Header
    text("FIELD SHOP", px + S_5, py + 44.0, TEXT_H1, FG);
    text(
        "Supplies for the deadzone",
        px + S_5,
        py + 66.0,
        TEXT_MICRO,
        FG3,
    );
    text_right(
        &format!("{} c", game.player.coins),
        px + pw - S_5,
        py + 44.0,
        TEXT_H2,
        ACCENT,
    );
    hairline(px + S_5, py + 84.0, pw - S_5 * 2.0);

    // Items
    let mx = mouse_position().0;
    let my = mouse_position().1;
    let row_h = 55.0;
    let row_gap = 3.0;
    let mut clicked: Option<usize> = None;

    for (i, item) in INGAME_SHOP.iter().enumerate() {
        let y = py + 100.0 + i as f32 * (row_h + row_gap);
        let rx = px + S_5;
        let row_w = pw - S_5 * 2.0;

        let hovered = mx >= rx && mx <= rx + row_w && my >= y && my <= y + row_h;
        let selected = *sel == i;
        let affordable = game.player.coins >= item.price;

        if selected || hovered {
            draw_rectangle(rx, y, row_w, row_h, SURFACE_HI);
        }
        if selected {
            draw_rectangle(rx, y, 2.0, row_h, ACCENT);
        }

        // Left: name + description
        let name_color = if affordable { FG } else { FG3 };
        text(item.name, rx + S_4, y + 20.0, TEXT_H3, name_color);
        text(item.desc, rx + S_4, y + 38.0, TEXT_MICRO, FG3);

        // Right: price + status
        let price_color = if affordable { ACCENT } else { FG3 };
        text_right(
            &format!("{} c", item.price),
            rx + row_w - S_4,
            y + 20.0,
            TEXT_BODY,
            price_color,
        );

        let owned = match item.kind {
            ItemKind::AmmoRefill => 0,
            ItemKind::Grenade => game.player.grenade_count,
            ItemKind::Landmine => game.player.landmine_count,
            ItemKind::Claymore => game.player.claymore_count,
            ItemKind::MedKit => 0,
            ItemKind::ArmorPlate => 0,
            ItemKind::Nuke => game.player.nuke_count,
        };
        if owned > 0 {
            text_right(
                &format!("owned: {}", owned),
                rx + row_w - S_4,
                y + 38.0,
                TEXT_MICRO,
                SUCCESS,
            );
        } else if affordable {
            text_right(
                "ENTER / CLICK",
                rx + row_w - S_4,
                y + 38.0,
                TEXT_MICRO,
                ACCENT,
            );
        }

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            clicked = Some(i);
        }
    }

    // Keyboard navigation
    if is_key_pressed(KeyCode::Down) {
        *sel = (*sel + 1) % INGAME_SHOP.len();
    }
    if is_key_pressed(KeyCode::Up) {
        *sel = (*sel + INGAME_SHOP.len() - 1) % INGAME_SHOP.len();
    }

    // Purchase
    let mut to_buy: Option<usize> = None;
    if is_key_pressed(KeyCode::Enter) {
        to_buy = Some(*sel);
    }
    if let Some(i) = clicked {
        *sel = i;
        to_buy = Some(i);
    }

    if let Some(i) = to_buy {
        let item = &INGAME_SHOP[i];
        if game.player.coins >= item.price {
            let mut applied = true;
            match item.kind {
                ItemKind::AmmoRefill => {
                    let target = game.player.current_mag() * 4;
                    if game.player.reserve >= target {
                        applied = false;
                    } else {
                        game.player.reserve = target;
                    }
                }
                ItemKind::Grenade => game.player.grenade_count += 1,
                ItemKind::MedKit => {
                    if game.player.health >= game.player.max_health {
                        applied = false;
                    } else {
                        game.player.health =
                            (game.player.health + 50.0).min(game.player.max_health);
                    }
                }
                ItemKind::ArmorPlate => {
                    if game.player.armor >= 50.0 {
                        applied = false;
                    } else {
                        game.player.armor = (game.player.armor + 25.0).min(50.0);
                    }
                }
                ItemKind::Landmine => game.player.landmine_count += 1,
                ItemKind::Claymore => game.player.claymore_count += 1,
                ItemKind::Nuke => {
                    if game.player.nuke_count >= 1 {
                        applied = false;
                    } else {
                        game.player.nuke_count += 1;
                    }
                }
            }
            if applied {
                game.player.coins -= item.price;
                audio::sfx_click();
            }
        }
    }

    // Footer
    let footer_y = py + ph - 26.0;
    hairline(px + S_5, footer_y - 20.0, pw - S_5 * 2.0);
    text(
        "UP/DOWN   |   ENTER OR CLICK TO BUY   |   B / ESC CLOSE",
        px + S_5,
        footer_y,
        TEXT_MICRO,
        FG3,
    );
}
