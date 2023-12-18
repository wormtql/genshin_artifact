use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct NighttimeWhispersInTheEchoingWoodsEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> ArtifactEffect<A> for NighttimeWhispersInTheEchoingWoodsEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("回声之林夜话2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        let bonus = self.rate1 * (0.2 + 0.3 * self.rate2);
        attribute.set_value_by(AttributeName::BonusGeo, "回声之林夜话4", bonus);
    }
}

pub struct NighttimeWhispersInTheEchoingWoods;

impl ArtifactTrait for NighttimeWhispersInTheEchoingWoods {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NighttimeWhispersInTheEchoingWoodsEffect {
            rate1: config.config_nighttime_whispers_in_the_echoing_woods.rate1,
            rate2: config.config_nighttime_whispers_in_the_echoing_woods.rate2
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NighttimeWhispersInTheEchoingWoods,
        name_mona: "NighttimeWhispersInTheEchoingWoods",
        name_locale: locale!(
            zh_cn: "回声之林夜话",
            en: "Nighttime Whispers in the Echoing Woods"
        ),
        flower: Some(locale!(
            zh_cn: "无私的妆饰花",
            en: "Selfless Floral Accessory"
        )),
        feather: Some(locale!(
            zh_cn: "诚恳的蘸水笔",
            en: "Honest Quill"
        )),
        sand: Some(locale!(
            zh_cn: "忠实的砂时计",
            en: "Faithful Hourglass"
        )),
        goblet: Some(locale!(
            zh_cn: "慷慨的墨水瓶",
            en: "Magnanimous Ink Bottle"
        )),
        head: Some(locale!(
            zh_cn: "慈爱的淑女帽",
            en: "Compassionate Ladies' Hat"
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "施放元素战技后的10秒内，岩元素伤害加成提升20%；若处于结晶反应产生的护盾庇护下，上述效果提高150%，进一步提高的效果将在失去结晶护盾庇护的1秒后移除。",
            en: "After using an Elemental Skill, gain a 20% Geo DMG Bonus for 10s. While under a shield granted by the Crystallize reaction, the above effect will be increased by 150%, and this additional increase disappears 1s after that shield is lost."
        )),
        effect5: None,
        internal_id: 15034
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "总被动比例",
                en: "Ratio1"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "护盾比例",
                en: "Shield Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
