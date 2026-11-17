use crate::systems::save::Save;
use crate::weapons::attachments::ATTACHMENTS;
use crate::weapons::weapon::WeaponCategory;

pub fn toggle_attachment(save: &mut Save, w_idx: usize, slot: usize, att_idx: usize) -> bool {
    if w_idx >= 9 || slot >= 5 {
        return false;
    }
    if att_idx >= ATTACHMENTS.len() {
        return false;
    }
    if !save.owned_attachments[att_idx] {
        return false;
    }
    if ATTACHMENTS[att_idx].slot != slot {
        return false;
    }
    if !crate::weapons::weapon::is_attachment_compatible(w_idx, att_idx) {
        return false;
    }

    if save.weapon_attachments[w_idx][slot] == att_idx as i32 {
        save.weapon_attachments[w_idx][slot] = -1;
    } else {
        save.weapon_attachments[w_idx][slot] = att_idx as i32;
    }
    true
}

pub fn set_loadout(save: &mut Save, idx: usize) -> bool {
    if idx >= 9 {
        return false;
    }
    if !save.owned_weapons[idx] {
        return false;
    }
    match WeaponCategory::of(idx) {
        WeaponCategory::Primary => {
            save.loadout_primary = idx;
            save.equipped_weapon = idx;
            true
        }
        WeaponCategory::Secondary => {
            save.loadout_secondary = idx;
            true
        }
        WeaponCategory::Melee => false,
    }
}
