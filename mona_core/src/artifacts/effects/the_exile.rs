use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct TheExileEffect;

impl<T: Attribute> ArtifactEffect<T> for TheExileEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "流放者2", 0.2);
    }
}

pub struct TheExile;

impl ArtifactTrait for TheExile {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(TheExileEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TheExile,
        name_mona: "exile",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "流放者",
            en: "The Exile",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "流放者之花",
            en: "Exile's Flower",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "流放者之羽",
            en: "Exile's Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "流放者怀表",
            en: "Exile's Pocket Watch",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "流放者之杯",
            en: "Exile's Goblet",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "流放者头冠",
            en: "Exile's Circlet",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素充能效率提高20%。",
            en: "Energy Recharge +20%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发后，每2秒为队伍中所有角色（不包括自己）恢复2点元素能量。该效果持续6秒，无法叠加。",
            en: "Using an Elemental Burst regenerates 2 Energy for all party members (excluding the wearer) every 2s for 6s. This effect cannot stack.",
        )),
        effect5: None,
        internal_id: 10009,
    };
}
