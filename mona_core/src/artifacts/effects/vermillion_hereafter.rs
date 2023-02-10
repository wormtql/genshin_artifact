use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct VermillionHereafterEffect {
    pub rate_q: f64,
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for VermillionHereafterEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("辰砂往生录2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        let bonus = self.rate_q * (0.08 + 0.1 * self.stack);
        attribute.add_atk_percentage("辰砂往生录4", bonus);
    }
}

pub struct VermillionHereafter;

impl ArtifactTrait for VermillionHereafter {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        let rate_q = config.config_vermillion_hereafter.rate_q;
        let stack = config.config_vermillion_hereafter.stack;

        Box::new(VermillionHereafterEffect {
            rate_q, stack
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::VermillionHereafter,
        name_mona: "VermillionHereafter",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "辰砂往生录",
            en: "Vermillion Hereafter",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "生灵之华",
            en: "Flowering Life",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "潜光片羽",
            en: "Feather of Nascent Light",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "阳辔之遗",
            en: "Solar Relic",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "结契之刻",
            en: "Moment of the Pact",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "虺雷之姿",
            en: "Thundering Poise",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发后，将产生持续16秒的「潜光」效果：攻击力提升8%；并在角色的生命值降低时，攻击力进一步提升10%，至多通过这种方式提升4次，每0.8秒至多触发一次。「潜光」效果将在角色退场时消失；持续期间再次施放元素爆发，将移除原有的「潜光」。",
            en: "After using an Elemental Burst, this character will gain the Nascent Light effect, increasing their ATK by 8% for 16s. When the character's HP decreases, their ATK will further increase by 10%. This further increase can occur this way a maximum of 4 times. This effect can be triggered once every 0.8s. Nascent Light will be dispelled when the character leaves the field. If an Elemental Burst is used again during the duration of Nascent Light, the original Nascent Light will be dispelled.",
        )),
        effect5: None,
        internal_id: 15023,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate_q",
            title: crate::common::i18n::locale!(
                zh_cn: "元素爆发频率",
                en: "Elemental Burst Rate",
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "stack",
            title: crate::common::i18n::locale!(
                zh_cn: "平均层数",
                en: "Avg Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        }
    ]);
}
