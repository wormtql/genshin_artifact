use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;

pub struct GladiatorsFinaleEffect {
    weapon_type: WeaponType,
}

impl GladiatorsFinaleEffect {
    pub fn new(weapon_type: WeaponType) -> GladiatorsFinaleEffect {
        GladiatorsFinaleEffect {
            weapon_type
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for GladiatorsFinaleEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("角斗士的终幕礼2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        match self.weapon_type {
            WeaponType::Sword | WeaponType::Claymore | WeaponType::Polearm => {
                attribute.set_value_by(AttributeName::BonusNormalAttack, "角斗士的终幕礼4", 0.35);
            },
            _ => (),
        };
    }
}

pub struct GladiatorsFinale;

impl ArtifactTrait for GladiatorsFinale {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GladiatorsFinaleEffect::new(character_common_data.static_data.weapon_type))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::GladiatorsFinale,
        name_mona: "gladiatorFinale",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "角斗士的终幕礼",
            en: "Gladiator's Finale",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "角斗士的留恋",
            en: "Gladiator's Nostalgia",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "角斗士的归宿",
            en: "Gladiator's Destiny",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "角斗士的希冀",
            en: "Gladiator's Longing",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "角斗士的酣醉",
            en: "Gladiator's Intoxication",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "角斗士的凯旋",
            en: "Gladiator's Triumphus",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "装备该圣遗物套装的角色为单手剑、双手剑、长柄武器角色时，角色普通攻击造成的伤害提高35%。",
            en: "If the wielder of this artifact set uses a Sword, Claymore or Polearm, increases their Normal Attack DMG by 35%.",
        )),
        effect5: None,
        internal_id: 15001,
    };
}
