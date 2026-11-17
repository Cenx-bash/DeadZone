use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use std::cell::RefCell;

const SR: u32 = 44100;

struct Lcg(u64);
impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_f32(&mut self) -> f32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 40) as f32 / 0xFFFFFF as f32) * 2.0 - 1.0
    }
}

fn wav_bytes(samples: &[i16]) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut wav = Vec::with_capacity(44 + data_len as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&((36 + data_len) as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&SR.to_le_bytes());
    wav.extend_from_slice(&(SR * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

fn push_normalized(out: &mut Vec<i16>, sample: f32, gain: f32) {
    out.push(((sample.clamp(-1.0, 1.0)) * gain) as i16);
}

fn synth_shot() -> Vec<u8> {
    let dur = 0.18;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(1);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-t * 18.0).exp();
        let noise = rng.next_f32();
        let thump = (t * 2.0 * std::f32::consts::PI * 85.0).sin() * (-t * 14.0).exp();
        let crack = (t * 2.0 * std::f32::consts::PI * 1800.0).sin() * (-t * 60.0).exp();
        let s = noise * 0.45 + thump * 0.45 + crack * 0.1;
        push_normalized(&mut out, s * env, 24000.0);
    }
    wav_bytes(&out)
}

fn synth_shotgun() -> Vec<u8> {
    let dur = 0.38;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(2);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-t * 6.5).exp();
        let noise = rng.next_f32();
        let boom = (t * 2.0 * std::f32::consts::PI * 48.0).sin() * (-t * 5.0).exp();
        let s = noise * 0.5 + boom * 0.5;
        push_normalized(&mut out, s * env, 26000.0);
    }
    wav_bytes(&out)
}

fn synth_knife() -> Vec<u8> {
    let dur = 0.12;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(3);
    let mut out = Vec::with_capacity(n);
    let mut last = 0.0f32;
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (1.0 - t / dur).max(0.0).powf(2.0);
        let noise = rng.next_f32();
        let hp = noise - last;
        last = noise;
        let whoosh = (t * 2.0 * std::f32::consts::PI * 300.0).sin() * (-t * 30.0).exp();
        let s = hp * 0.7 + whoosh * 0.3;
        push_normalized(&mut out, s * env, 16000.0);
    }
    wav_bytes(&out)
}

fn synth_hit() -> Vec<u8> {
    let dur = 0.10;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(4);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-t * 45.0).exp();
        let noise = rng.next_f32();
        let tone = (t * 2.0 * std::f32::consts::PI * 220.0).sin() * (-t * 20.0).exp();
        let s = noise * 0.4 + tone * 0.6;
        push_normalized(&mut out, s * env, 20000.0);
    }
    wav_bytes(&out)
}

fn synth_headshot() -> Vec<u8> {
    let dur = 0.16;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(5);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-t * 28.0).exp();
        let noise = rng.next_f32();
        let tone = (t * 2.0 * std::f32::consts::PI * 460.0).sin() * (-t * 15.0).exp();
        let s = noise * 0.35 + tone * 0.65;
        push_normalized(&mut out, s * env, 22000.0);
    }
    wav_bytes(&out)
}

fn synth_reload() -> Vec<u8> {
    let dur = 0.22;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(6);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let e1 = (-(t.abs() * 90.0)).exp();
        let e2 = (-((t - 0.10).abs() * 90.0)).exp();
        let noise = rng.next_f32();
        let tone = (t * 2.0 * std::f32::consts::PI * 950.0).sin();
        let s = (e1 + e2) * tone * 0.5 + noise * 0.25 * (e1 + e2);
        push_normalized(&mut out, s * 0.5, 18000.0);
    }
    wav_bytes(&out)
}

fn synth_pickup() -> Vec<u8> {
    let dur = 0.20;
    let n = (SR as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let freq = 640.0 + t * 900.0;
        let env = (-t * 8.0).exp();
        let s = (t * 2.0 * std::f32::consts::PI * freq).sin() * env * 0.5;
        push_normalized(&mut out, s, 20000.0);
    }
    wav_bytes(&out)
}

fn synth_hurt() -> Vec<u8> {
    let dur = 0.25;
    let n = (SR as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let freq = (240.0 - t * 400.0).max(60.0);
        let env = (-t * 6.0).exp();
        let s = (t * 2.0 * std::f32::consts::PI * freq).sin() * env * 0.6;
        push_normalized(&mut out, s, 20000.0);
    }
    wav_bytes(&out)
}

fn synth_explosion() -> Vec<u8> {
    let dur = 0.75;
    let n = (SR as f32 * dur) as usize;
    let mut rng = Lcg::new(7);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-t * 3.5).exp();
        let noise = rng.next_f32();
        let boom = (t * 2.0 * std::f32::consts::PI * 35.0).sin() * (-t * 2.5).exp();
        let s = noise * 0.45 + boom * 0.55;
        push_normalized(&mut out, s * env, 26000.0);
    }
    wav_bytes(&out)
}

fn synth_click() -> Vec<u8> {
    let dur = 0.04;
    let n = (SR as f32 * dur) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / SR as f32;
        let env = (-t * 120.0).exp();
        let s = (t * 2.0 * std::f32::consts::PI * 1400.0).sin() * env * 0.4;
        push_normalized(&mut out, s, 14000.0);
    }
    wav_bytes(&out)
}

struct AudioBank {
    shot: Option<Sound>,
    shotgun: Option<Sound>,
    knife: Option<Sound>,
    hit: Option<Sound>,
    headshot: Option<Sound>,
    reload: Option<Sound>,
    pickup: Option<Sound>,
    hurt: Option<Sound>,
    explosion: Option<Sound>,
    ui_click: Option<Sound>,
}

impl AudioBank {
    async fn load() -> Self {
        let mut failures = Vec::<&'static str>::new();

        async fn try_load(
            bytes: Vec<u8>,
            name: &'static str,
            failures: &mut Vec<&'static str>,
        ) -> Option<Sound> {
            match load_sound_from_bytes(&bytes).await {
                Ok(s) => Some(s),
                Err(e) => {
                    eprintln!("[audio] failed to load '{}': {:?}", name, e);
                    failures.push(name);
                    None
                }
            }
        }

        let shot = try_load(synth_shot(), "shot", &mut failures).await;
        let shotgun = try_load(synth_shotgun(), "shotgun", &mut failures).await;
        let knife = try_load(synth_knife(), "knife", &mut failures).await;
        let hit = try_load(synth_hit(), "hit", &mut failures).await;
        let headshot = try_load(synth_headshot(), "headshot", &mut failures).await;
        let reload = try_load(synth_reload(), "reload", &mut failures).await;
        let pickup = try_load(synth_pickup(), "pickup", &mut failures).await;
        let hurt = try_load(synth_hurt(), "hurt", &mut failures).await;
        let explosion = try_load(synth_explosion(), "explosion", &mut failures).await;
        let ui_click = try_load(synth_click(), "ui_click", &mut failures).await;

        if failures.is_empty() {
            eprintln!("[audio] all 10 sounds loaded successfully");
        } else {
            eprintln!(
                "[audio] {} of 10 sounds failed to load: {:?}",
                failures.len(),
                failures
            );
            eprintln!(
                "[audio] audio may be unavailable. Check that PulseAudio/PipeWire is running."
            );
        }

        Self {
            shot,
            shotgun,
            knife,
            hit,
            headshot,
            reload,
            pickup,
            hurt,
            explosion,
            ui_click,
        }
    }
}

thread_local! {
    static BANK: RefCell<Option<AudioBank>> = RefCell::new(None);
    static VOLUME: RefCell<f32> = RefCell::new(0.7);
}

pub async fn init() {
    let bank = AudioBank::load().await;
    BANK.with(|b| *b.borrow_mut() = Some(bank));
}

pub fn set_volume(v: f32) {
    VOLUME.with(|v_| *v_.borrow_mut() = v.clamp(0.0, 1.0));
}

pub fn is_loaded() -> bool {
    BANK.with(|b| b.borrow().is_some())
}

fn play_opt(sound: &Option<Sound>, scale: f32) {
    let master = VOLUME.with(|v| *v.borrow());
    let vol = (master * scale).clamp(0.0, 1.0);
    if vol <= 0.001 {
        return;
    }
    if let Some(s) = sound {
        // macroquad 0.4.x: play_sound returns () — the backend manages playback lifetime
        play_sound(
            s,
            PlaySoundParams {
                looped: false,
                volume: vol,
            },
        );
    }
}

fn dispatch<F: FnOnce(&AudioBank)>(f: F) {
    BANK.with(|b| {
        if let Some(bank) = b.borrow().as_ref() {
            f(bank);
        }
    });
}

pub fn sfx_shot() {
    dispatch(|b| play_opt(&b.shot, 0.70));
}
pub fn sfx_shotgun() {
    dispatch(|b| play_opt(&b.shotgun, 0.95));
}
pub fn sfx_knife() {
    dispatch(|b| play_opt(&b.knife, 0.70));
}
pub fn sfx_hit() {
    dispatch(|b| play_opt(&b.hit, 0.65));
}
pub fn sfx_headshot() {
    dispatch(|b| play_opt(&b.headshot, 0.90));
}
pub fn sfx_reload() {
    dispatch(|b| play_opt(&b.reload, 0.75));
}
pub fn sfx_pickup() {
    dispatch(|b| play_opt(&b.pickup, 0.70));
}
pub fn sfx_hurt() {
    dispatch(|b| play_opt(&b.hurt, 0.90));
}
pub fn sfx_explosion() {
    dispatch(|b| play_opt(&b.explosion, 1.00));
}
pub fn sfx_click() {
    dispatch(|b| play_opt(&b.ui_click, 0.40));
}
