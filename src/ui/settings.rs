use macroquad::prelude::*;

use crate::systems::audio;
use crate::systems::settings::Settings;
use crate::ui::theme::*;
use crate::State;

pub fn draw_settings(settings: &mut Settings, sel: &mut usize) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    let margin = S_8;

    draw_rectangle(0.0, 0.0, sw, sh, BG);

    text("SETTINGS", margin, 96.0, TEXT_H1, FG);
    text("Preferences", margin, 120.0, TEXT_MICRO, FG3);

    // Audio status indicator
    let (audio_label, audio_color) = if audio::is_loaded() {
        ("AUDIO: ONLINE", SUCCESS)
    } else {
        ("AUDIO: UNAVAILABLE", DANGER)
    };
    text_right(audio_label, sw - margin, 120.0, TEXT_MICRO, audio_color);

    hairline(margin, 136.0, 280.0);

    let rows = 4;
    if is_key_pressed(KeyCode::Down) {
        *sel = (*sel + 1) % rows;
    }
    if is_key_pressed(KeyCode::Up) {
        *sel = (*sel + rows - 1) % rows;
    }

    let list_y = 200.0;
    let list_w = 420.0;
    let row_h = 52.0;
    let row_gap = 2.0;

    let mx = mouse_position().0;
    let my = mouse_position().1;

    for i in 0..rows {
        let y = list_y + i as f32 * (row_h + row_gap);
        let focused = *sel == i;
        let hovered = mx >= margin && mx <= margin + list_w && my >= y && my <= y + row_h;
        let active = focused || hovered;

        if active {
            draw_rectangle(margin, y, list_w, row_h, SURFACE);
        }
        if focused {
            draw_rectangle(margin + S_4, y + row_h / 2.0 - 2.0, 4.0, 4.0, ACCENT);
        }

        let (label, value) = match i {
            0 => (
                "MASTER VOLUME",
                format!("{:.0}%", settings.master_volume * 100.0),
            ),
            1 => (
                "MOUSE SENSITIVITY",
                format!("{:.2}", settings.sensitivity * 1000.0),
            ),
            2 => ("FIELD OF VIEW", format!("{:.0}°", settings.fov)),
            _ => ("BACK", String::new()),
        };

        text(
            label,
            margin + S_5,
            y + 32.0,
            TEXT_H3,
            if active { FG } else { FG2 },
        );
        if !value.is_empty() {
            text_right(
                &value,
                margin + list_w - S_4,
                y + 32.0,
                TEXT_BODY,
                if active { ACCENT } else { FG3 },
            );
        }

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            *sel = i;
            if i == 3 {
                return Some(State::Menu);
            }
        }
    }

    let adjust = if is_key_pressed(KeyCode::Left) {
        -1
    } else if is_key_pressed(KeyCode::Right) {
        1
    } else {
        0
    };
    if adjust != 0 {
        match *sel {
            0 => {
                settings.master_volume =
                    (settings.master_volume + adjust as f32 * 0.05).clamp(0.0, 1.0);
                audio::set_volume(settings.master_volume);
                audio::sfx_click();
            }
            1 => {
                settings.sensitivity =
                    (settings.sensitivity + adjust as f32 * 0.0005).clamp(0.0005, 0.015);
            }
            2 => {
                settings.fov = (settings.fov + adjust as f32 * 5.0).clamp(60.0, 110.0);
            }
            _ => {}
        }
    }

    if is_key_pressed(KeyCode::Enter) && *sel == 3 {
        return Some(State::Menu);
    }

    let footer_y = sh - margin;
    hairline(margin, footer_y - 30.0, sw - margin * 2.0);
    text(
        "UP/DOWN NAVIGATE   |   LEFT/RIGHT ADJUST   |   ENTER CONFIRM   |   ESC BACK",
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
