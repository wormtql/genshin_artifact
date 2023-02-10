use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct CrimsonWitchOfFlamesEffect {
    pub level: f64,
}

impl CrimsonWitchOfFlamesEffect {
    pub fn new(config: &ArtifactEffectConfig) -> CrimsonWitchOfFlamesEffect {
        CrimsonWitchOfFlamesEffect {
            level: config.config_crimson_witch_of_flames.level.min(3.0),
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for CrimsonWitchOfFlamesEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPyro, "炽烈的炎之魔女2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        let key = "炽烈的炎之魔女4";
        attribute.set_value_by(AttributeName::EnhanceOverload, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceBurning, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceBurgeon, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceVaporize, key, 0.15);
        attribute.set_value_by(AttributeName::EnhanceMelt, key, 0.15);
        attribute.set_value_by(AttributeName::BonusPyro, key, self.level * 0.075);
    }
}

pub struct CrimsonWitchOfFlames;

impl ArtifactTrait for CrimsonWitchOfFlames {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(CrimsonWitchOfFlamesEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::CrimsonWitchOfFlames,
        name_mona: "crimsonWitch",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "炽烈的炎之魔女",
            en: "Crimson Witch of Flames",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "魔女的炎之花",
            en: "Witch's Flower of Blaze",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "魔女常燃之羽",
            en: "Witch's Ever-Burning Plume",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "魔女破灭之时",
            en: "Witch's End Time",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "魔女的心之火",
            en: "Witch's Heart Flames",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "焦灼的魔女帽",
            en: "Witch's Scorching Hat",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%火元素伤害加成。",
            en: "Pyro DMG Bonus +15%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "超载、燃烧反应造成的伤害提升40%，蒸发、融化反应的加成系数提高15%。施放元素战技后的10秒内，2件套的效果提高50%，该效果最多叠加3次。",
            en: "Increases Overloaded and Burning DMG by 40%. Increases Vaporize and Melt DMG by 15%. Using Elemental Skill increases the 2-Piece Set Bonus by 50% of its starting value for 10s. Max 3 stacks.",
        )),
        effect5: None,
        internal_id: 15006,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: crate::common::i18n::locale!(
                zh_cn: "效果等效层数",
                en: "Equivalent Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);
}
