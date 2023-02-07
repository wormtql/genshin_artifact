use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct EmblemOfSeveredFateEffect;

impl<T: Attribute> ArtifactEffect<T> for EmblemOfSeveredFateEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "绝缘之旗印2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_edge1(
            AttributeName::Recharge,
            AttributeName::BonusElementalBurst,
            Box::new(|x, _| (x * 0.25).min(0.75)),
            Box::new(|grad, x, _| (if x < 3.0 { grad * 0.25 } else { 0.0 }, 0.0)),
            "绝缘之旗印4"
        );
    }
}

pub struct EmblemOfSeveredFate;

impl ArtifactTrait for EmblemOfSeveredFate {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(EmblemOfSeveredFateEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::EmblemOfSeveredFate,
        name_mona: "emblemOfSeveredFate",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "绝缘之旗印",
            en: "Emblem of Severed Fate",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "明威之镡",
            en: "Magnificent Tsuba",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "切落之羽",
            en: "Sundered Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "雷云之笼",
            en: "Storm Cage",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "绯花之壶",
            en: "Scarlet Vessel",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "华饰之兜",
            en: "Ornate Kabuto",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素充能效率提高20%。",
            en: "Energy Recharge +20%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "基于元素充能效率的25%，提高元素爆发造成的伤害。至多通过这种方式获得75%提升。",
            en: "Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum of 75% bonus DMG can be obtained in this way.",
        )),
        effect5: None,
        internal_id: 15020,
    };
}
