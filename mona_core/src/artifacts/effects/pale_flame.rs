use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct PaleFlameEffect {
    pub level: f64,
    pub full_rate: f64,
}

impl PaleFlameEffect {
    pub fn new(config: &ArtifactEffectConfig) -> PaleFlameEffect {
        PaleFlameEffect {
            level: config.config_pale_flame.avg_level,
            full_rate: config.config_pale_flame.full_rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for PaleFlameEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "苍白之火2", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("苍白之火4", 0.09 * self.level);
        attribute.set_value_by(AttributeName::BonusPhysical, "苍白之火4", 0.25 * self.full_rate);
    }
}

pub struct PaleFlame;

impl ArtifactTrait for PaleFlame {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(PaleFlameEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::PaleFlame,
        name_mona: "paleFlame",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "苍白之火",
            en: "Pale Flame",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "无垢之花",
            en: "Stainless Bloom",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "贤医之羽",
            en: "Wise Doctor's Pinion",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "停摆之刻",
            en: "Moment of Cessation",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "超越之盏",
            en: "Surpassing Cup",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "嗤笑之面",
            en: "Mocking Mask",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "造成的物理伤害提高25%。",
            en: "Physical DMG is increased by 25%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技命中敌人后，攻击力提升9%。该效果持续7秒，至多叠加2层，每0.3秒至多触发一次。叠满2层时，2件套的效果提升100%。",
            en: "When an Elemental Skill hits an opponent, ATK is increased by 9% for 7s. This effect stacks up to 2 times and can be triggered once every 0.3s. Once 2 stacks are reached, the 2-set effect is increased by 100%.",
        )),
        effect5: None,
        internal_id: 15018,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "avg_level",
            title: crate::common::i18n::locale!(
                zh_cn: "效果1等效层数",
                en: "Effect1 Equivalent Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        },
        ItemConfig {
            name: "full_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "满层效果比例",
                en: "Full Stack Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
