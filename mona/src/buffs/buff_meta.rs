use strum_macros::Display;
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::weapon::WeaponName;

pub enum BuffImage {
    Custom(&'static str),
    Avatar(CharacterName),
    Misc(&'static str),
}

#[derive(Display)]
pub enum BuffGenre {
    Common,
    Character,
    Weapon,
    Artifact,
    Resonance,
}

#[derive(Display)]
pub enum BuffFrom {
    Character(CharacterName),
    Weapon(WeaponName),
    Common,
    Resonance,
}

pub struct BuffMetaData {
    pub name: BuffName,
    pub chs: &'static str,
    pub image: BuffImage,
    pub genre: BuffGenre,
    pub description: Option<&'static str>,
    pub from: BuffFrom,
}