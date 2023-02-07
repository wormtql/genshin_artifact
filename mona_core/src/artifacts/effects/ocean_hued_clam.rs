use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct OceanHuedClamEffect;

impl<T: Attribute> ArtifactEffect<T> for OceanHuedClamEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HealingBonus, "海染砗磲2", 0.15);
    }

    // todo
    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}

pub struct OceanHuedClam;

impl ArtifactTrait for OceanHuedClam {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(OceanHuedClamEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::OceanHuedClam,
        name_mona: "oceanHuedClam",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "海染砗磲",
            en: "Ocean-Hued Clam",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "海染之花",
            en: "Sea-Dyed Blossom",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "渊宫之羽",
            en: "Deep Palace's Plume",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "离别之贝",
            en: "Cowry of Parting",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "真珠之笼",
            en: "Pearl Cage",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "海祇之冠",
            en: "Crown of Watatsumi",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "治疗加成提高15%。",
            en: "Healing Bonus +15%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "装备此圣遗物套装的角色对队伍中的角色进行治疗时，将产生持续3秒的海染泡沫，记录治疗的生命值回复量（包括溢出值）。持续时间结束时，海染泡沫将会爆炸，对周围的敌人造成90%累计回复量的伤害（该伤害结算方式同感电、超导等元素反应，但不受元素精通、等级或反应伤害加成效果影响）。每3.5秒至多产生一个海染泡沫；海染泡沫至多记录30000点回复量，含溢出部分的治疗量；自己的队伍中同时至多存在一个海染泡沫。装备此圣遗物套装的角色处于队伍后台时，依然能触发该效果。",
            en: "When the character equipping this artifact set heals a character in the party, a Sea-Dyed Foam will appear for 3 seconds, accumulating the amount of HP recovered from healing (including overflow healing). At the end of the duration, the Sea-Dyed Foam will explode, dealing DMG to nearby opponents based on 90% of the accumulated healing. (This DMG is calculated similarly to Reactions such as Electro-Charged, and Superconduct, but is not affected by Elemental Mastery, Character Levels, or Reaction DMG Bonuses). Only one Sea-Dyed Foam can be produced every 3.5 seconds. Each Sea-Dyed Foam can accumulate up to 30,000 HP (including overflow healing). There can be no more than one Sea-Dyed Foam active at any given time. This effect can still be triggered even when the character who is using this artifact set is not on the field.",
        )),
        effect5: None,
        internal_id: 15022,
    };
}
