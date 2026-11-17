use crate::maps::arena::{Crate, Pillar, HALF, PILLAR_R};
use macroquad::prelude::*;

pub fn push_out(pos: &mut Vec3, r: f32, pillars: &[Pillar], crates: &[Crate]) {
    for p in pillars {
        let d = vec2(pos.x - p.x, pos.z - p.z);
        let min = PILLAR_R + r;
        let len = d.length();
        if len < min {
            let n = if len > 0.001 { d / len } else { vec2(1.0, 0.0) };
            pos.x = p.x + n.x * min;
            pos.z = p.z + n.y * min;
        }
    }

    for c in crates {
        if c.hp <= 0.0 {
            continue;
        }
        if pos.y >= c.h {
            continue;
        }

        let closest_x = pos.x.clamp(c.x - c.hw, c.x + c.hw);
        let closest_z = pos.z.clamp(c.z - c.hd, c.z + c.hd);
        let dx = pos.x - closest_x;
        let dz = pos.z - closest_z;
        let dist_sq = dx * dx + dz * dz;

        if dist_sq < r * r && dist_sq > 1e-6 {
            let dist = dist_sq.sqrt();
            let push = (r - dist) / dist;
            pos.x += dx * push;
            pos.z += dz * push;
        } else if dist_sq <= 1e-6 {
            let to_left = (pos.x - (c.x - c.hw)).abs();
            let to_right = ((c.x + c.hw) - pos.x).abs();
            let to_top = (pos.z - (c.z - c.hd)).abs();
            let to_bottom = ((c.z + c.hd) - pos.z).abs();
            let m = to_left.min(to_right).min(to_top).min(to_bottom);
            if m == to_left {
                pos.x = c.x - c.hw - r;
            } else if m == to_right {
                pos.x = c.x + c.hw + r;
            } else if m == to_top {
                pos.z = c.z - c.hd - r;
            } else {
                pos.z = c.z + c.hd + r;
            }
        }
    }

    let lim = HALF - r;
    pos.x = pos.x.clamp(-lim, lim);
    pos.z = pos.z.clamp(-lim, lim);
}
