pub mod attachments;
pub mod combat;
pub mod weapon;

pub use attachments::{compute_mods, AttachmentDef, ATTACHMENTS, SLOT_NAMES};
pub use combat::{FloatingText, Grenade, Particle, Shell, Tracer};
pub use weapon::{Rarity, WeaponDef, WEAPONS};
