use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;

pub struct WanderersTroupeEffect {
    pub weapon_type: WeaponType
}

impl WanderersTroupeEffect {
    pub fn new(weapon_type: WeaponType) -> WanderersTroupeEffect {
        WanderersTroupeEffect {
            weapon_type,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for WanderersTroupeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "流浪大地的乐团2", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        match self.weapon_type {
            WeaponType::Catalyst | WeaponType::Bow => {
                attribute.set_value_by(AttributeName::BonusChargedAttack, "流浪大地的乐团4", 0.35);
            },
            _ => (),
        }
    }
}

pub struct WanderersTroupe;

impl ArtifactTrait for WanderersTroupe {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(WanderersTroupeEffect::new(character_common_data.static_data.weapon_type))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::WanderersTroupe,
        name_mona: "wandererTroupe",
        chs: "流浪大地的乐团",
        flower: Some("乐团的晨光"),
        feather: Some("琴师的箭羽"),
        sand: Some("终幕的时计"),
        goblet: Some("吟游者之壶"),
        head: Some("指挥的礼帽"),
        star: (4, 5),
        effect1: None,
        effect2: Some("元素精通提高80点。"),
        effect3: None,
        effect4: Some("装备该圣遗物套装的角色为法器、弓箭角色时，角色重击造成的伤害提高35%。"),
        effect5: None
    };
}
