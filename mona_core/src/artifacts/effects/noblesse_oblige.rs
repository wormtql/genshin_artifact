use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct NoblesseObligeEffect {
    pub rate: f64,
}

impl NoblesseObligeEffect {
    pub fn new(config: &ArtifactEffectConfig) -> NoblesseObligeEffect {
        NoblesseObligeEffect {
            rate: config.config_noblesse_oblige.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for NoblesseObligeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalBurst, "昔日宗室之仪2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("昔日宗室之仪4", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "昔日宗室之仪4", self.rate * 0.2);
    }
}

pub struct NoblesseOblige;

impl ArtifactTrait for NoblesseOblige {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NoblesseObligeEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NoblesseOblige,
        name_mona: "noblesseOblige",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "昔日宗室之仪",
            en: "Noblesse Oblige",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "宗室之花",
            en: "Royal Flora",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "宗室之翎",
            en: "Royal Plume",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "宗室时计",
            en: "Royal Pocket Watch",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "宗室银瓮",
            en: "Royal Silver Urn",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "宗室面具",
            en: "Royal Masque",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素爆发造成的伤害提升20%。",
            en: "Elemental Burst DMG +20%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发后，队伍中所有角色攻击力提升20%，持续12秒。该效果不可叠加。",
            en: "Using an Elemental Burst increases all party members' ATK by 20% for 12s. This effect cannot stack.",
        )),
        effect5: None,
        internal_id: 15007,
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
