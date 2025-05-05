use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

struct LongNightsOathEffect {
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for LongNightsOathEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "长夜之誓2", 0.25);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "长夜之誓4", self.stack * 0.15);
    }
}

pub struct LongNightsOath;

impl ArtifactTrait for LongNightsOath {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(LongNightsOathEffect {
            stack: config.config_long_nights_oath.level
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::LongNightsOath,
        name_mona: "LongNightsOath",
        name_locale: locale!(
            zh_cn: "长夜之誓",
            en: "Long Night’s Oath"
        ),
        flower: Some(locale!(
            zh_cn: "执灯人的誓词",
            en: "Lightkeeper's Pledge"
        )),
        feather: Some(locale!(
            zh_cn: "夜鸣莺的尾羽",
            en: "Nightingale's Tail Feather"
        )),
        sand: Some(locale!(
            zh_cn: "不死者的哀铃",
            en: "Undying One's Mourning Bell"
        )),
        goblet: Some(locale!(
            zh_cn: "未吹响的号角",
            en: "A Horn Unwinded"
        )),
        head: Some(locale!(
            zh_cn: "被浸染的缨盔",
            en: "Dyed Tassel"
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "下落攻击造成的伤害提升25%。",
            en: "Plunging Attack DMG increased by 25%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "装备者的下落攻击/重击/元素战技命中敌人后，获得1/2/2层「永照的流辉」，由下落攻击、重击或元素战技产生的该效果分别每1秒至多触发一次。永照的流辉：下落攻击造成的伤害提升15%，持续6秒，至多叠加5层，每层持续时间独立计算。",
            en: "After the equipping character’s Plunging Attack/Charged Attack/Elemental Skill hits an opponent, they will gain 1/2/2 stack(s) of “Radiance Everlasting.” Plunging Attacks, Charged Attacks, or Elemental Skills can each trigger this effect once every 1s. Radiance Everlasting: Plunging Attacks deal 15% increased DMG for 6s. Max 5 stacks. Each stack’s duration is counted independently."
        )),
        effect5: None,
        internal_id: 15039,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: locale!(
                zh_cn: "层数",
                en: "Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 }
        }
    ]);
}
