use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct MarechausseeHunterEffect {
    pub stack: f64
}

impl<A: Attribute> ArtifactEffect<A> for MarechausseeHunterEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "逐影猎人2", 0.15);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "逐影猎人2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalBase, "逐影猎人4", 0.12 * self.stack);
    }
}

pub struct MarechausseeHunter;

impl ArtifactTrait for MarechausseeHunter {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(MarechausseeHunterEffect {
            stack: config.config_marechaussee_hunter.stack
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::MarechausseeHunter,
        name_mona: "MarechausseeHunter",
        name_locale: locale!(
            zh_cn: "逐影猎人",
            en: "Marechaussee Hunter"
        ),
        flower: Some(locale!(zh_cn: "猎人的胸花", en: "Hunter's Brooch")),
        feather: Some(locale!(zh_cn: "杰作的序曲", en: "Masterpiece's Overture")),
        sand: Some(locale!(zh_cn: "裁判的时刻", en: "Moment of Judgment")),
        goblet: Some(locale!(zh_cn: "遗忘的容器", en: "Forgotten Vessel")),
        head: Some(locale!(zh_cn: "老兵的容颜", en: "Veteran's Visage")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "普通攻击与重击造成的伤害提高15%。",
            en: "Normal and Charged Attack DMG +15%"
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "当前生命值提升或降低时，暴击率提升12%，该效果持续5秒，至多叠加3次。",
            en: "When current HP increases or decreases, CRIT Rate will be increased by 12% for 5s. Max 3 stacks."
        )),
        effect5: None,
        internal_id: 15031
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "平均层数",
                en: "Avg Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);
}
