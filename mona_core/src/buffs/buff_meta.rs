use strum_macros::Display;
use crate::artifacts::ArtifactSetName;
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::I18nLocale;
use crate::weapon::WeaponName;

pub enum BuffImage {
    Custom(&'static str),
    Avatar(CharacterName),
    Weapon(WeaponName),
    Misc(&'static str),
    Artifact(ArtifactSetName),
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
    Artifact(ArtifactSetName),
    Common,
    Resonance,
}

pub struct BuffMetaData {
    pub name: BuffName,
    pub name_locale: I18nLocale,
    pub image: BuffImage,
    pub genre: BuffGenre,
    pub description: Option<I18nLocale>,
    pub from: BuffFrom,
}