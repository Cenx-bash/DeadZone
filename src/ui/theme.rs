use macroquad::prelude::*;

// ---- Palette ----
pub const BG: Color = Color::new(0.035, 0.035, 0.043, 1.0);
pub const SURFACE: Color = Color::new(0.070, 0.070, 0.082, 1.0);
pub const SURFACE_HI: Color = Color::new(0.098, 0.098, 0.110, 1.0);
pub const BORDER: Color = Color::new(0.118, 0.122, 0.141, 1.0);
pub const BORDER_HI: Color = Color::new(0.165, 0.173, 0.196, 1.0);

pub const FG: Color = Color::new(0.918, 0.918, 0.925, 1.0);
pub const FG2: Color = Color::new(0.541, 0.545, 0.565, 1.0);
pub const FG3: Color = Color::new(0.353, 0.357, 0.380, 1.0);
pub const FG4: Color = Color::new(0.208, 0.212, 0.239, 1.0);

pub const ACCENT: Color = Color::new(1.000, 0.380, 0.220, 1.0);
pub const ACCENT_DIM: Color = Color::new(1.000, 0.380, 0.220, 0.14);

pub const DANGER: Color = Color::new(0.929, 0.298, 0.298, 1.0);
pub const SUCCESS: Color = Color::new(0.427, 0.816, 0.494, 1.0);
pub const WARNING: Color = Color::new(0.965, 0.757, 0.376, 1.0);
pub const INFO: Color = Color::new(0.400, 0.706, 1.000, 1.0);

// ---- Type scale ----
pub const TEXT_DISPLAY: f32 = 64.0;
pub const TEXT_H1: f32 = 36.0;
pub const TEXT_H2: f32 = 24.0;
pub const TEXT_H3: f32 = 18.0;
pub const TEXT_BODY: f32 = 15.0;
pub const TEXT_SMALL: f32 = 13.0;
pub const TEXT_MICRO: f32 = 11.0;

// ---- Spacing ----
pub const S_1: f32 = 4.0;
pub const S_2: f32 = 8.0;
pub const S_3: f32 = 12.0;
pub const S_4: f32 = 16.0;
pub const S_5: f32 = 24.0;
pub const S_6: f32 = 32.0;
pub const S_7: f32 = 48.0;
pub const S_8: f32 = 64.0;

// ---- Text helpers ----
pub fn text(s: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(s, x, y, size, color);
}

pub fn text_right(s: &str, x_right: f32, y: f32, size: f32, color: Color) {
    let w = measure_text(s, None, size as u16, 1.0).width;
    draw_text(s, x_right - w, y, size, color);
}

pub fn text_center(s: &str, cx: f32, y: f32, size: f32, color: Color) {
    let w = measure_text(s, None, size as u16, 1.0).width;
    draw_text(s, cx - w / 2.0, y, size, color);
}

pub fn center_text(s: &str, y: f32, size: f32, color: Color) {
    text_center(s, screen_width() / 2.0, y, size, color);
}

// ---- Structural primitives ----
pub fn hairline(x: f32, y: f32, w: f32) {
    draw_rectangle(x, y, w, 1.0, BORDER);
}

pub fn bar(x: f32, y: f32, w: f32, h: f32, pct: f32, color: Color) {
    draw_rectangle(x, y, w, h, SURFACE);
    draw_rectangle(x, y, w * pct.clamp(0.0, 1.0), h, color);
}

pub fn section_header(x: f32, y: f32, w: f32, label: &str) {
    text(label, x, y, TEXT_MICRO, FG3);
    hairline(x, y + S_2, w);
}

pub fn card(x: f32, y: f32, w: f32, h: f32, selected: bool, hovered: bool) {
    let bg = if selected || hovered {
        SURFACE_HI
    } else {
        SURFACE
    };
    draw_rectangle(x, y, w, h, bg);
    let border = if selected {
        ACCENT
    } else if hovered {
        BORDER_HI
    } else {
        BORDER
    };
    draw_rectangle_lines(x, y, w, h, 1.0, border);
    if selected {
        draw_rectangle(x, y, 2.0, h, ACCENT);
    }
}

pub fn stat_row(x: f32, y: f32, w: f32, label: &str, value: &str, value_color: Color) {
    text(label, x, y, TEXT_MICRO, FG3);
    text_right(value, x + w, y, TEXT_BODY, value_color);
}
