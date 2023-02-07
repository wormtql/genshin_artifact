use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ScholarEffect;

impl<T: Attribute> ArtifactEffect<T> for ScholarEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "学士2", 0.2);
    }
}

pub struct Scholar;

impl ArtifactTrait for Scholar {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ScholarEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Scholar,
        name_mona: "scholar",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "学士",
            en: "Scholar",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "学士的书签",
            en: "Scholar's Bookmark",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "学士的羽笔",
            en: "Scholar's Quill Pen",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "学士的时钟",
            en: "Scholar's Clock",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "学士的墨杯",
            en: "Scholar's Ink Cup",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "学士的镜片",
            en: "Scholar's Lens",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素充能效率提高20%。",
            en: "Energy Recharge +20%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "获得元素微粒或元素晶球时，队伍中所有弓箭和法器角色额外恢复3点元素能量。该效果每3秒只能触发一次。",
            en: "Gaining Elemental Particles or Orbs gives 3 Energy to all party members who have a bow or a catalyst equipped. Can only occur once every 3s.",
        )),
        effect5: None,
        internal_id: 10012,
    };
}
