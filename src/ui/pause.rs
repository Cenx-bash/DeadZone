use macroquad::prelude::*;

use crate::systems::save::Save;
use crate::ui::theme::*;
use crate::State;

pub fn draw_pause() -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.75));

    let cy = sh / 2.0;
    text_center("PAUSED", sw / 2.0, cy - 20.0, TEXT_DISPLAY, FG);
    text_center(
        "ESC to resume   |   Q to quit",
        sw / 2.0,
        cy + 40.0,
        TEXT_BODY,
        FG2,
    );

    if is_key_pressed(KeyCode::Escape) {
        return Some(State::Play);
    }
    if is_key_pressed(KeyCode::Q) {
        return Some(State::Menu);
    }
    None
}

pub fn draw_dead(save: &mut Save, score: u32, kills: u32, wave: u32) -> Option<State> {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.05, 0.02, 0.02, 0.92));

    text_center("YOU DIED", sw / 2.0, sh * 0.30, TEXT_DISPLAY, DANGER);

    let stats_y = sh * 0.44;
    let row_h = 32.0;

    text_center(&format!("WAVE {}", wave), sw / 2.0, stats_y, TEXT_H2, FG);
    text_center(
        &format!("{} KILLS", kills),
        sw / 2.0,
        stats_y + row_h,
        TEXT_H2,
        FG,
    );
    text_center(
        &format!("{} SCORE", score),
        sw / 2.0,
        stats_y + row_h * 2.0,
        TEXT_H2,
        FG,
    );

    text_center(
        &format!("BEST WAVE {}", save.best_wave),
        sw / 2.0,
        stats_y + row_h * 3.0 + S_5,
        TEXT_BODY,
        ACCENT,
    );

    text_center("ENTER to continue", sw / 2.0, sh * 0.85, TEXT_BODY, FG2);

    if is_key_pressed(KeyCode::Enter) {
        return Some(State::Menu);
    }
    None
}
