mod game;
mod maps;
mod systems;
mod ui;
mod weapons;
mod zombies;

use macroquad::prelude::*;

use game::Game;
use maps::arena::maps;
use systems::audio;
use systems::save::Save;
use ui::abilities::draw_abilities;
use ui::inventory::draw_inventory;
use ui::menu::{draw_loading, draw_menu, MenuCmd};
use ui::pause::{draw_dead, draw_pause};
use ui::settings::draw_settings;
use ui::shop::draw_shop;
use ui::shop_ingame::draw_field_shop;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Loading,
    Menu,
    Shop,
    Inventory,
    Abilities,
    Settings,
    Play,
    Pause,
    FieldShop,
    Dead,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Deadzone: Zombie Survival".to_owned(),
        window_width: 1280,
        window_height: 720,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    audio::init().await;

    let mut save = Save::load_from_disk();
    audio::set_volume(save.settings.master_volume);

    let mut state = State::Loading;
    let mut load_t = 0.0f32;

    let mut menu_sel = 0usize;
    let mut shop_sel = 0usize;
    let mut inv_weapon = 0usize;
    let mut inv_slot = 0usize;
    let mut inv_att = 0usize;
    let mut abil_sel = 0usize;
    let mut settings_sel = 0usize;
    let mut ingame_sel = 0usize;
    let mut map_idx = 0usize;

    let mut game = Game::new(&save, map_idx);
    let mut last_mouse: Vec2 = mouse_position().into();
    let mut grabbed = false;
    let mut prev_wave_for_best = 0u32;

    loop {
        let dt = get_frame_time().min(0.05);
        let mp: Vec2 = mouse_position().into();
        let mouse_delta = mp - last_mouse;
        last_mouse = mp;
        let wheel_y = mouse_wheel().1;

        clear_background(game.fog);

        match state {
            State::Loading => {
                load_t += dt;
                draw_loading(load_t);
                if load_t > 1.8 {
                    state = State::Menu;
                }
            }
            State::Menu => {
                game.draw_world();
                set_default_camera();
                match draw_menu(&save, &mut menu_sel, &mut map_idx) {
                    Some(MenuCmd::Play) => {
                        game = Game::new(&save, map_idx);
                        prev_wave_for_best = 0;
                        state = State::Play;
                    }
                    Some(MenuCmd::Shop) => state = State::Shop,
                    Some(MenuCmd::Inventory) => state = State::Inventory,
                    Some(MenuCmd::Abilities) => state = State::Abilities,
                    Some(MenuCmd::Settings) => {
                        settings_sel = 0;
                        state = State::Settings;
                    }
                    Some(MenuCmd::CycleMap) => {
                        map_idx = (map_idx + 1) % maps().len();
                        game = Game::new(&save, map_idx);
                    }
                    Some(MenuCmd::Quit) => break,
                    None => {}
                }
            }
            State::Settings => {
                if let Some(ns) = draw_settings(&mut save.settings, &mut settings_sel) {
                    save.save_to_disk();
                    state = ns;
                }
            }
            State::Shop => {
                if let Some(ns) = draw_shop(&mut save, &mut shop_sel) {
                    state = ns;
                }
            }
            State::Inventory => {
                if let Some(ns) =
                    draw_inventory(&mut save, &mut inv_weapon, &mut inv_slot, &mut inv_att)
                {
                    state = ns;
                }
            }
            State::Abilities => {
                if let Some(ns) = draw_abilities(&mut save, &mut abil_sel) {
                    state = ns;
                }
            }
            State::Play => {
                game.update(dt, mouse_delta, &save, wheel_y);
                game.draw_world();
                set_default_camera();
                game.draw_hud();

                if game.waves.wave > prev_wave_for_best {
                    prev_wave_for_best = game.waves.wave;
                }

                if game.player.health <= 0.0 {
                    save.coins = game.player.coins;
                    save.total_kills += game.kills;
                    if prev_wave_for_best > save.best_wave {
                        save.best_wave = prev_wave_for_best;
                    }
                    save.save_to_disk();
                    state = State::Dead;
                } else if is_key_pressed(KeyCode::Escape) {
                    state = State::Pause;
                } else if is_key_pressed(KeyCode::B) {
                    ingame_sel = 0;
                    state = State::FieldShop;
                }
            }
            State::Pause => {
                game.draw_world();
                set_default_camera();
                if let Some(ns) = draw_pause() {
                    if ns == State::Menu {
                        save.coins = game.player.coins;
                        if prev_wave_for_best > save.best_wave {
                            save.best_wave = prev_wave_for_best;
                        }
                        save.save_to_disk();
                    }
                    state = ns;
                }
            }
            State::FieldShop => {
                game.draw_world();
                set_default_camera();
                game.draw_hud();
                draw_field_shop(&mut game, &mut ingame_sel);

                if is_key_pressed(KeyCode::B) || is_key_pressed(KeyCode::Escape) {
                    state = State::Play;
                }
            }
            State::Dead => {
                if let Some(ns) = draw_dead(&mut save, game.score, game.kills, game.waves.wave) {
                    state = ns;
                }
            }
        }

        let want_grab = state == State::Play;
        if want_grab != grabbed {
            let _ = set_cursor_grab(want_grab);
            show_mouse(!want_grab);
            grabbed = want_grab;
            last_mouse = mouse_position().into();
        }

        next_frame().await;
    }

    save.save_to_disk();
}
