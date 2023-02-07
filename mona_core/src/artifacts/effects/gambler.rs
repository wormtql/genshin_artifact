use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct GamblerEffect;

impl<T: Attribute> ArtifactEffect<T> for GamblerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalSkill, "赌徒2", 0.2);
    }
}

pub struct Gambler;

impl ArtifactTrait for Gambler {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GamblerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Gambler,
        name_mona: "gambler",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "赌徒",
            en: "Gambler",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "赌徒的胸花",
            en: "Gambler's Brooch",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "赌徒的羽饰",
            en: "Gambler's Feather Accessory",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "赌徒的怀表",
            en: "Gambler's Pocket Watch",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "赌徒的骰盅",
            en: "Gambler's Dice Cup",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "赌徒的耳环",
            en: "Gambler's Earrings",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技造成的伤害提升20%。",
            en: "Increases Elemental Skill DMG by 20%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "击败敌人时，有100%概率清除元素战技的冷却时间。该效果每15秒至多触发一次。",
            en: "Defeating an opponent has a 100% chance to remove Elemental Skill CD. Can only occur once every 15s.",
        )),
        effect5: None,
        internal_id: 10008,
    };
}
