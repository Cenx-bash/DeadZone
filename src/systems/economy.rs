pub const WAVE_BONUS_PER_WAVE: u32 = 50;

pub fn wave_clear_bonus(wave: u32) -> u32 {
    WAVE_BONUS_PER_WAVE * wave
}
