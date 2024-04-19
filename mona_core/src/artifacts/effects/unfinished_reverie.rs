use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct UnfinishedReverieEffect {
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for UnfinishedReverieEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("未竟的遐思2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        let bonus = 0.5 * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "未竟的遐思4", bonus);
    }
}

pub struct UnfinishedReverie;

impl ArtifactTrait for UnfinishedReverie {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(UnfinishedReverieEffect {
            rate: config.config_unfinished_reverie.rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::UnfinishedReverie,
        name_mona: "UnfinishedReverie",
        name_locale: locale!(zh_cn: "未竟的遐思", en: "Unfinished Reverie"),
        flower: Some(locale!(zh_cn: "暗结的明花", en: "Dark Fruit of Bright Flowers")),
        feather: Some(locale!(zh_cn: "褪光的翠尾", en: "Faded Emerald Tail")),
        sand: Some(locale!(zh_cn: "举业的识刻", en: "Moment of Attainment")),
        goblet: Some(locale!(zh_cn: "筹谋的共樽", en: "The Wine-Flask Over Which the Plan Was Hatched")),
        head: Some(locale!(zh_cn: "失冕的宝冠", en: "Crownless Crown")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%"
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "脱离战斗状态3秒后，造成的伤害提升50%。在战斗状态下，附近不存在处于燃烧状态下的敌人超过6秒后，上述伤害提升效果每秒降低10%，直到降低至0%；存在处于燃烧状态下的敌人时，每秒提升10%，直到达到50%。装备此圣遗物套装的角色处于队伍后台时，依然会触发该效果。",
            en: "After leaving combat for 3s, DMG dealt increased by 50%. In combat, if no Burning opponents are nearby for more than 6s, this DMG Bonus will decrease by 10% per second until it reaches 0%. When a Burning opponent exists, it will increase by 10% instead until it reaches 50%. This effect still triggers if the equipping character is off-field."
        )),
        effect5: None,
        internal_id: 15036,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "被动比例",
                en: "Effect Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
