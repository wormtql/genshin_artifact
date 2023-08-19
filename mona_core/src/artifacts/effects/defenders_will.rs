use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct DefendersWillEffect;

impl<T: Attribute> ArtifactEffect<T> for DefendersWillEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("守护之心2", 0.3);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}

pub struct DefendersWill;

impl ArtifactTrait for DefendersWill {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DefendersWillEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DefendersWill,
        name_mona: "defenderWill",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "守护之心",
            en: "Defender's Will",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "守护之花",
            en: "Guardian's Flower",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "守护徽印",
            en: "Guardian's Sigil",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "守护座钟",
            en: "Guardian's Clock",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "守护之皿",
            en: "Guardian's Vessel",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "守护束带",
            en: "Guardian's Band",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "防御力提高30%。",
            en: "DEF +30%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "队伍里每有不同一种元素类型的自己的角色，自身获得30%相应的元素抗性。",
            en: "For each different element present in your own party, the wearer's Elemental RES to that corresponding element is increased by 30%.",
        )),
        effect5: None,
        internal_id: 10003,
    };
}
