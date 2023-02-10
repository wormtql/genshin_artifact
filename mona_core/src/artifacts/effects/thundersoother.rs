use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct ThundersootherEffect {
    pub rate: f64,
}

impl ThundersootherEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ThundersootherEffect {
        ThundersootherEffect {
            rate: config.config_thundersoother.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ThundersootherEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "平息鸣雷的尊者4", self.rate * 0.35);
    }
}

pub struct Thundersoother;

impl ArtifactTrait for Thundersoother {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ThundersootherEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Thundersoother,
        name_mona: "thunderSmoother",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "平息鸣雷的尊者",
            en: "Thundersoother",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "平雷之心",
            en: "Thundersoother's Heart",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "平雷之羽",
            en: "Thundersoother's Plume",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "平雷之刻",
            en: "Hour of Soothing Thunder",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "平雷之器",
            en: "Thundersoother's Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "平雷之冠",
            en: "Thundersoother's Diadem",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "雷元素抗性提高40%。",
            en: "Electro RES increased by 40%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "对处于雷元素影响下的敌人造成的伤害提升35%。",
            en: "Increases DMG against opponents affected by Electro by 35%.",
        )),
        effect5: None,
        internal_id: 14002,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "敌方雷元素覆盖率",
                en: "Enemy Electro Coverage",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
