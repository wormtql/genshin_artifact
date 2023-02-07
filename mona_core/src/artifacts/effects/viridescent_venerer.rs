use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ViridescentVenererEffect;

impl<T: Attribute> ArtifactEffect<T> for ViridescentVenererEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusAnemo, "翠绿之影2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceSwirlBase, "翠绿之影4", 0.6);
    }
}

pub struct ViridescentVenerer;

impl ArtifactTrait for ViridescentVenerer {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ViridescentVenererEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ViridescentVenerer,
        name_mona: "viridescentVenerer",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "翠绿之影",
            en: "Viridescent Venerer",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "野花记忆的绿野",
            en: "In Remembrance of Viridescent Fields",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "猎人青翠的箭羽",
            en: "Viridescent Arrow Feather",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "翠绿猎人的笃定",
            en: "Viridescent Venerer's Determination",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "翠绿猎人的容器",
            en: "Viridescent Venerer's Vessel",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "翠绿的猎人之冠",
            en: "Viridescent Venerer's Diadem",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%风元素伤害加成。",
            en: "Anemo DMG Bonus +15%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "扩散反应造成的伤害提升60%。根据扩散的元素类型，降低受到影响的敌人40%的对应元素抗性，持续10秒。",
            en: "Increases Swirl DMG by 60%. Decreases opponent's Elemental RES to the element infused in the Swirl by 40% for 10s.",
        )),
        effect5: None,
        internal_id: 15002,
    };
}
