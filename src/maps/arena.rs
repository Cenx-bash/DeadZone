use macroquad::prelude::*;

pub const HALF: f32 = 24.0;
pub const PILLAR_R: f32 = 1.1;

/// Walls use this HP so grenades/bullets never chew through them.
const WALL_HP: f32 = 1.0e9;

#[derive(Clone)]
pub struct Pillar {
    pub x: f32,
    pub z: f32,
}

#[derive(Clone)]
pub struct Crate {
    pub x: f32,
    pub z: f32,
    pub hw: f32,
    pub hd: f32,
    pub h: f32,
    pub hp: f32,
}

pub struct MapDef {
    pub name: &'static str,
    pub tagline: &'static str,
    pub pillars: Vec<Pillar>,
    pub crates: Vec<Crate>,
    pub floor: Color,
    pub tile: Color,
    pub wall: Color,
    pub wall_trim: Color,
    pub accent: Color,
    pub fog: Color,
}

/// Destructible cover.
fn crate_(x: f32, z: f32, hw: f32, hd: f32, h: f32) -> Crate {
    Crate {
        x,
        z,
        hw,
        hd,
        h,
        hp: 40.0,
    }
}

/// Indestructible structure (walls, building shells, barriers).
fn wall(x: f32, z: f32, hw: f32, hd: f32, h: f32) -> Crate {
    Crate {
        x,
        z,
        hw,
        hd,
        h,
        hp: WALL_HP,
    }
}

fn pillar(x: f32, z: f32) -> Pillar {
    Pillar { x, z }
}

pub fn maps() -> Vec<MapDef> {
    vec![
        // ============================================================
        // DUSTBOWL — Desert, three-lane layout in the spirit of a
        // classic bomb-defusal map:
        //   EAST  : Long lane running into the A site (boxes + goose)
        //   CENTER: Mid corridor with door frames, wide open center
        //   WEST  : Narrow B tunnels spilling into the B site, with
        //           a window wall looking back at mid
        // Every lane has a different feel: long sightline, medium
        // choke, tight tunnel.
        // ============================================================
        MapDef {
            name: "DUSTBOWL",
            tagline: "Long A / Mid / B Tunnels",
            pillars: vec![
                // Mid door frames
                pillar(-5.5, -3.5),
                pillar(5.5, -3.5),
                pillar(-5.5, 3.5),
                pillar(5.5, 3.5),
                // Long-A corner and tunnel mouth
                pillar(15.0, -4.2),
                pillar(-13.5, -6.5),
            ],
            crates: vec![
                // --- Mid corridor walls (open gap in the middle) ---
                wall(-5.5, -10.0, 0.6, 5.5, 3.0),
                wall(5.5, -10.0, 0.6, 5.5, 3.0),
                wall(-5.5, 10.0, 0.6, 5.5, 3.0),
                wall(5.5, 10.0, 0.6, 5.5, 3.0),
                // --- Long A lane wall ---
                wall(15.0, -13.0, 0.6, 7.0, 3.0),
                // --- B tunnels (two parallel walls, 4 wide) ---
                wall(-20.5, -14.0, 0.5, 6.0, 3.0),
                wall(-15.5, -14.0, 0.5, 6.0, 3.0),
                // --- B window wall ---
                wall(-9.0, 12.0, 0.5, 3.5, 2.4),
                // --- Mid cover ---
                crate_(0.0, -12.0, 1.2, 1.0, 1.2),
                crate_(0.0, 12.0, 1.2, 1.0, 1.2),
                crate_(-10.0, -3.0, 1.0, 1.0, 1.0),
                // --- Long A lane cover ---
                crate_(20.0, -12.0, 1.0, 1.0, 1.1),
                crate_(19.0, -18.0, 1.0, 1.0, 1.2),
                crate_(20.0, -3.0, 1.0, 1.0, 0.9),
                // --- A site ---
                crate_(17.0, 6.0, 1.8, 1.8, 1.6), // default box
                crate_(21.0, 10.0, 1.1, 1.1, 1.1),
                crate_(13.0, 10.0, 1.0, 1.0, 1.0), // goose
                // --- B site ---
                crate_(-17.0, 10.0, 1.8, 1.8, 1.6),
                crate_(-21.0, 14.0, 1.0, 1.0, 1.1),
                crate_(-13.0, 6.0, 1.0, 1.0, 1.0),
                crate_(-16.0, 18.0, 1.2, 0.8, 1.0),
                // --- Tunnel mouth cover ---
                crate_(-18.0, -5.5, 1.0, 1.0, 1.0),
            ],
            floor: Color::new(0.26, 0.20, 0.13, 1.0),
            tile: Color::new(0.20, 0.15, 0.09, 1.0),
            wall: Color::new(0.52, 0.39, 0.22, 1.0),
            wall_trim: Color::new(0.92, 0.73, 0.40, 1.0),
            accent: Color::new(0.95, 0.76, 0.42, 1.0),
            fog: Color::new(0.22, 0.15, 0.08, 1.0),
        },
        // ============================================================
        // COMPOUND — Olive-drab close quarters, two-house standoff.
        // West house has windows and a wide door, east house is a
        // solid bunker with one big entrance. Between them: a
        // courtyard with a bus, a car and lamp posts.
        // ============================================================
        MapDef {
            name: "COMPOUND",
            tagline: "Twin Houses",
            pillars: vec![
                // Lamp posts / yard posts
                pillar(0.0, -17.0),
                pillar(0.0, 17.0),
                pillar(-6.5, -4.0),
                pillar(6.5, 4.0),
                pillar(-8.0, 16.0),
                pillar(8.0, -16.0),
            ],
            crates: vec![
                // --- WEST HOUSE (x -20..-11, z -7..7) ---
                wall(-20.0, 0.0, 0.5, 7.5, 3.0), // back wall
                // North side with a window gap
                wall(-18.5, -7.0, 1.1, 0.5, 3.0),
                wall(-13.0, -7.0, 2.0, 0.5, 3.0),
                // South side with a window gap
                wall(-18.5, 7.0, 1.1, 0.5, 3.0),
                wall(-13.0, 7.0, 2.0, 0.5, 3.0),
                // Front wall, big door in the middle
                wall(-11.0, -5.0, 0.5, 2.0, 3.0),
                wall(-11.0, 5.0, 0.5, 2.0, 3.0),
                // Interior furniture
                crate_(-17.0, -3.0, 1.2, 1.2, 1.0),
                crate_(-17.0, 3.5, 1.0, 1.5, 1.0),
                crate_(-14.0, 0.0, 0.8, 0.8, 0.8),
                // --- EAST HOUSE (x 11..20, z -8..8) ---
                wall(20.0, 0.0, 0.5, 8.0, 3.0), // back wall
                wall(15.5, -8.0, 4.5, 0.5, 3.0),
                wall(15.5, 8.0, 4.5, 0.5, 3.0),
                wall(11.0, -6.0, 0.5, 2.5, 3.0),
                wall(11.0, 6.0, 0.5, 2.5, 3.0),
                // Interior
                crate_(16.0, 0.0, 1.5, 1.5, 1.2),
                crate_(14.0, -5.0, 1.0, 1.0, 1.0),
                crate_(14.0, 5.0, 1.0, 1.0, 1.0),
                // --- COURTYARD ---
                crate_(0.0, -10.0, 3.0, 1.2, 1.6), // bus
                crate_(-3.0, 10.0, 2.0, 1.0, 1.3), // car
                crate_(5.0, 13.0, 1.0, 1.0, 1.0),
                crate_(-6.0, -14.0, 1.0, 1.0, 1.0),
                // Low fences north / south
                crate_(-12.0, -16.0, 2.0, 0.6, 0.9),
                crate_(12.0, 16.0, 2.0, 0.6, 0.9),
                crate_(12.0, -16.0, 2.0, 0.6, 0.9),
                crate_(-12.0, 16.0, 2.0, 0.6, 0.9),
            ],
            floor: Color::new(0.13, 0.16, 0.11, 1.0),
            tile: Color::new(0.10, 0.12, 0.08, 1.0),
            wall: Color::new(0.30, 0.38, 0.24, 1.0),
            wall_trim: Color::new(0.65, 0.80, 0.40, 1.0),
            accent: Color::new(0.75, 0.90, 0.45, 1.0),
            fog: Color::new(0.06, 0.09, 0.05, 1.0),
        },
        // ============================================================
        // COLOSSEUM — Gated arena. A walled inner ring with four
        // gates, corner pillars, and an outer ring of towers,
        // pillars and low blocks. Fight in the pit or flank outside.
        // ============================================================
        MapDef {
            name: "COLOSSEUM",
            tagline: "Gated Arena",
            pillars: vec![
                // Inner ring corners
                pillar(-9.0, -9.0),
                pillar(9.0, -9.0),
                pillar(-9.0, 9.0),
                pillar(9.0, 9.0),
                // Outer ring
                pillar(-14.0, -14.0),
                pillar(14.0, -14.0),
                pillar(-14.0, 14.0),
                pillar(14.0, 14.0),
            ],
            crates: vec![
                // --- Inner ring, four 6-wide gates ---
                wall(-6.0, -9.0, 3.0, 0.6, 2.0),
                wall(6.0, -9.0, 3.0, 0.6, 2.0),
                wall(-6.0, 9.0, 3.0, 0.6, 2.0),
                wall(6.0, 9.0, 3.0, 0.6, 2.0),
                wall(-9.0, -6.0, 0.6, 3.0, 2.0),
                wall(-9.0, 6.0, 0.6, 3.0, 2.0),
                wall(9.0, -6.0, 0.6, 3.0, 2.0),
                wall(9.0, 6.0, 0.6, 3.0, 2.0),
                // --- Inside the pit ---
                crate_(-4.5, -4.5, 0.9, 0.9, 1.0),
                crate_(4.5, -4.5, 0.9, 0.9, 1.0),
                crate_(-4.5, 4.5, 0.9, 0.9, 1.0),
                crate_(4.5, 4.5, 0.9, 0.9, 1.0),
                // --- Corner towers ---
                crate_(-19.0, -19.0, 2.2, 2.2, 2.6),
                crate_(19.0, -19.0, 2.2, 2.2, 2.6),
                crate_(-19.0, 19.0, 2.2, 2.2, 2.6),
                crate_(19.0, 19.0, 2.2, 2.2, 2.6),
                // --- Mid-edge cover ---
                crate_(17.0, 0.0, 0.8, 2.5, 1.4),
                crate_(-17.0, 0.0, 0.8, 2.5, 1.4),
                crate_(0.0, 17.0, 2.5, 0.8, 1.4),
                crate_(0.0, -17.0, 2.5, 0.8, 1.4),
                // --- Low blocks in the outer ring ---
                crate_(13.0, -5.5, 0.7, 0.7, 0.8),
                crate_(13.0, 5.5, 0.7, 0.7, 0.8),
                crate_(-13.0, -5.5, 0.7, 0.7, 0.8),
                crate_(-13.0, 5.5, 0.7, 0.7, 0.8),
                crate_(-5.5, 13.0, 0.7, 0.7, 0.8),
                crate_(5.5, 13.0, 0.7, 0.7, 0.8),
                crate_(-5.5, -13.0, 0.7, 0.7, 0.8),
                crate_(5.5, -13.0, 0.7, 0.7, 0.8),
            ],
            floor: Color::new(0.16, 0.12, 0.20, 1.0),
            tile: Color::new(0.12, 0.09, 0.16, 1.0),
            wall: Color::new(0.34, 0.25, 0.46, 1.0),
            wall_trim: Color::new(0.70, 0.50, 0.90, 1.0),
            accent: Color::new(0.80, 0.60, 0.95, 1.0),
            fog: Color::new(0.09, 0.06, 0.13, 1.0),
        },
        // ============================================================
        // DOCKYARD — Shipping containers in staggered rows make
        // lanes of different widths; crane legs line the edges, a
        // loading ramp sits south of center and a three-sided
        // office offers elevated-feeling cover in the southeast.
        // ============================================================
        MapDef {
            name: "DOCKYARD",
            tagline: "Container Maze",
            pillars: vec![
                // Crane legs
                pillar(-21.0, -3.0),
                pillar(-21.0, 3.0),
                pillar(21.0, -3.0),
                pillar(21.0, 3.0),
                pillar(0.0, -21.0),
                pillar(0.0, 21.0),
            ],
            crates: vec![
                // --- North container row (z = -15), gaps of 3 and 8 and 3 ---
                crate_(-17.0, -15.0, 3.0, 1.2, 2.6),
                crate_(-8.0, -15.0, 3.0, 1.2, 2.6),
                crate_(6.0, -15.0, 3.0, 1.2, 2.6),
                crate_(15.0, -15.0, 3.0, 1.2, 2.6),
                // --- Standing containers west / east ---
                crate_(-14.0, -5.0, 1.2, 3.0, 2.6),
                crate_(11.0, -6.0, 1.2, 3.0, 2.6),
                // --- South container row, mixed heights ---
                crate_(-12.0, 15.0, 3.0, 1.2, 2.6),
                crate_(-3.0, 16.0, 3.0, 1.2, 1.3),
                crate_(9.0, 15.0, 3.0, 1.2, 2.6),
                // --- Loading ramp ---
                crate_(0.0, 9.0, 4.0, 0.8, 1.0),
                // --- Pallets ---
                crate_(-6.0, -9.5, 0.8, 0.8, 0.8),
                crate_(2.5, -9.5, 0.8, 0.8, 0.8),
                crate_(-18.0, 8.0, 0.9, 0.9, 0.9),
                // --- Office (open on the south side) ---
                wall(14.0, 10.0, 0.5, 3.0, 3.0),
                wall(18.0, 7.0, 4.0, 0.5, 3.0),
                crate_(19.0, 11.0, 1.0, 1.0, 1.0),
            ],
            floor: Color::new(0.13, 0.14, 0.16, 1.0),
            tile: Color::new(0.09, 0.10, 0.12, 1.0),
            wall: Color::new(0.50, 0.30, 0.20, 1.0),
            wall_trim: Color::new(0.95, 0.60, 0.25, 1.0),
            accent: Color::new(0.97, 0.62, 0.27, 1.0),
            fog: Color::new(0.07, 0.07, 0.09, 1.0),
        },
        // ============================================================
        // GLACIER — Pinwheel. Two nested rings of offset walls make
        // a swirling layout: every wall has an open side, so there
        // are no safe corners and no dead ends, just constant
        // flanking angles. Ice blocks fill the quadrants.
        // ============================================================
        MapDef {
            name: "GLACIER",
            tagline: "Pinwheel",
            pillars: vec![
                // Inner arm tips
                pillar(-12.5, -9.0),
                pillar(9.0, -12.5),
                pillar(12.5, 9.0),
                pillar(-9.0, 12.5),
                // Outer arm tips
                pillar(11.5, -17.0),
                pillar(17.0, 11.5),
                pillar(-11.5, 17.0),
                pillar(-17.0, -11.5),
            ],
            crates: vec![
                // --- Inner pinwheel ---
                wall(-6.0, -9.0, 6.0, 0.6, 2.8),
                wall(9.0, -6.0, 0.6, 6.0, 2.8),
                wall(6.0, 9.0, 6.0, 0.6, 2.8),
                wall(-9.0, 6.0, 0.6, 6.0, 2.8),
                // --- Outer pinwheel ---
                wall(5.0, -17.0, 6.0, 0.6, 2.8),
                wall(17.0, 5.0, 0.6, 6.0, 2.8),
                wall(-5.0, 17.0, 6.0, 0.6, 2.8),
                wall(-17.0, -5.0, 0.6, 6.0, 2.8),
                // --- Ice blocks in the quadrants ---
                crate_(-15.0, -13.0, 1.4, 1.4, 1.2),
                crate_(14.0, -12.0, 1.4, 1.4, 1.2),
                crate_(14.0, 13.0, 1.4, 1.4, 1.2),
                crate_(-14.0, 12.0, 1.4, 1.4, 1.2),
                // --- Low cover inside the swirl ---
                crate_(5.0, -5.0, 0.9, 0.9, 0.9),
                crate_(-5.0, 5.0, 0.9, 0.9, 0.9),
                // --- Corner glaciers ---
                crate_(-20.0, -20.0, 2.0, 2.0, 1.8),
                crate_(20.0, -20.0, 2.0, 2.0, 1.8),
                crate_(-20.0, 20.0, 2.0, 2.0, 1.8),
                crate_(20.0, 20.0, 2.0, 2.0, 1.8),
            ],
            floor: Color::new(0.15, 0.19, 0.24, 1.0),
            tile: Color::new(0.11, 0.15, 0.20, 1.0),
            wall: Color::new(0.40, 0.55, 0.70, 1.0),
            wall_trim: Color::new(0.75, 0.92, 1.00, 1.0),
            accent: Color::new(0.70, 0.90, 1.00, 1.0),
            fog: Color::new(0.08, 0.12, 0.17, 1.0),
        },
    ]
}
