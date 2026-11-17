use crate::game::Game;
use crate::systems::economy::wave_clear_bonus;

pub struct WaveManager {
    pub wave: u32,
    pub to_spawn: u32,
    pub spawn_t: f32,
    pub wave_break: f32,
}

impl WaveManager {
    pub fn new() -> Self {
        Self {
            wave: 0,
            to_spawn: 0,
            spawn_t: 0.0,
            wave_break: 2.5,
        }
    }
}

impl Game {
    pub(crate) fn update_wave(&mut self, dt: f32) {
        if self.waves.to_spawn > 0 {
            self.waves.spawn_t -= dt;
            if self.waves.spawn_t <= 0.0 {
                let w = self.waves.wave;
                self.spawn_zombie(w);
                self.waves.to_spawn -= 1;
                self.waves.spawn_t = (1.2 - w as f32 * 0.05).max(0.25);
            }
        } else if self.enemies.is_empty() {
            self.waves.wave_break -= dt;
            if self.waves.wave_break <= 0.0 {
                self.waves.wave += 1;
                let w = self.waves.wave;
                self.waves.to_spawn = 5 + w * 2;
                self.waves.wave_break = 4.0;
                self.player.health = (self.player.health + 30.0).min(self.player.max_health);
                self.player.coins += wave_clear_bonus(w);
                self.player.grenade_count += 1;
            }
        }
    }
}
