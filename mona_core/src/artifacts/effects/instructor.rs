use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct InstructorEffect {
    pub rate: f64,
}

impl InstructorEffect {
    pub fn new(config: &ArtifactEffectConfig) -> InstructorEffect {
        InstructorEffect {
            rate: config.config_instructor.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for InstructorEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "教官2", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "教官4", self.rate * 120.0);
    }
}

pub struct Instructor;

impl ArtifactTrait for Instructor {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(InstructorEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Instructor,
        name_mona: "instructor",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "教官",
            en: "Instructor",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "教官的胸花",
            en: "Instructor's Brooch",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "教官的羽饰",
            en: "Instructor's Feather Accessory",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "教官的怀表",
            en: "Instructor's Pocket Watch",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "教官的茶杯",
            en: "Instructor's Tea Cup",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "教官的帽子",
            en: "Instructor's Cap",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提高80点。",
            en: "Increases Elemental Mastery by 80.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "触发元素反应后，队伍中所有角色的元素精通提高120点，持续8秒。",
            en: "Upon triggering an Elemental Reaction, increases all party members' Elemental Mastery by 120 for 8s.",
        )),
        effect5: None,
        internal_id: 10007,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
