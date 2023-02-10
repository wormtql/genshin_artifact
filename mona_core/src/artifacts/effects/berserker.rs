use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BerserkerEffect {
    pub rate: f64,
}

impl BerserkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BerserkerEffect {
        BerserkerEffect {
            rate: config.config_berserker.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BerserkerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "战狂2", 0.12);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    }
}

pub struct Berserker;

impl ArtifactTrait for Berserker {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BerserkerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Berserker,
        name_mona: "berserker",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "战狂",
            en: "Berserker",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "战狂的蔷薇",
            en: "Berserker's Rose",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "战狂的翎羽",
            en: "Berserker's Indigo Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "战狂的时计",
            en: "Berserker's Timepiece",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "战狂的骨杯",
            en: "Berserker's Bone Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "战狂的鬼面",
            en: "Berserker's Battle Mask",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "暴击率提高12%。",
            en: "CRIT Rate +12%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "生命值低于70%时，暴击率额外提升24%。",
            en: "When HP is below 70%, CRIT Rate increases by an additional 24%.",
        )),
        effect5: None,
        internal_id: 10005,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
