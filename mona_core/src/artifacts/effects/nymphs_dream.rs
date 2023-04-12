use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct NymphsDreamEffect {
    pub w1: f64,
    pub w2: f64,
    pub w3: f64,
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for NymphsDreamEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusHydro, "水仙之梦2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        let weights = self.w1 + self.w2 + self.w3;
        if weights == 0.0 {
            return;
        }

        let r1 = self.w1 / weights;
        let r2 = self.w2 / weights;
        let r3 = self.w3 / weights;

        let atk_bonus = 0.07 * r1 + 0.16 * r2 + 0.25 * r3;
        let bonus_hydro = 0.04 * r1 + 0.09 * r2 + 0.15 * r3;

        attribute.add_atk_percentage("水仙之梦4", atk_bonus * self.rate);
        attribute.set_value_by(AttributeName::BonusHydro, "水仙之梦4", bonus_hydro * self.rate);
    }
}

pub struct NymphsDream;

impl ArtifactTrait for NymphsDream {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NymphsDreamEffect {
            w1: config.config_nymphs_dream.w1,
            w2: config.config_nymphs_dream.w2,
            w3: config.config_nymphs_dream.w3,
            rate: config.config_nymphs_dream.rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NymphsDream,
        name_mona: "NymphsDream",
        name_locale: locale!(zh_cn: "水仙之梦", en: "Nymph's Dream"),
        flower: Some(locale!(zh_cn: "旅途中的鲜花", en: "Odyssean Flower")),
        feather: Some(locale!(zh_cn: "坏巫师的羽杖", en: "Wicked Mage's Plumule")),
        sand: Some(locale!(zh_cn: "水仙的时时刻刻", en: "Nymph's Constancy")),
        goblet: Some(locale!(zh_cn: "勇者们的茶会", en: "Heroes' Tea Party")),
        head: Some(locale!(zh_cn: "恶龙的单片镜", en: "Fell Dragon's Monocle")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "获得15%水元素伤害加成。",
            en: "Hydro DMG Bonus +15%"
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "普通攻击、重击、下落攻击、元素战技或元素爆发命中敌人后，将产生1层持续8秒的「镜中水仙」效果。处于1/2/3层及以上「镜中水仙」效果下时，攻击力将提高7%/16%/25%，水元素伤害加成提升4%/9%/15%。由普通攻击、重击、下落攻击、元素战技或元素爆发产生的「镜中水仙」将分别独立存在。",
            en: "After Normal, Charged, and Plunging Attacks, Elemental Skills, and Elemental Bursts hit opponents, 1 stack of Mirrored Nymph will be triggered, lasting 8s. When under the effect of 1, 2, or 3 or more Mirrored Nymph stacks, ATK will be increased by 7%/16%/25%, and Hydro DMG Bonus will be increased by 4%/9%/15%. Mirrored Nymph stacks created by Normal, Charged, and Plunging Attacks, Elemental Skills, and Elemental Bursts exist independently."
        )),
        effect5: None,
        internal_id: 15029
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "w1",
            title: locale!(zh_cn: "一层权重", en: "Stack-1 Weight"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "w2",
            title: locale!(zh_cn: "二层权重", en: "Stack-2 Weight"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "w3",
            title: locale!(zh_cn: "三层权重", en: "Stack-3 Weight"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "rate",
            title: locale!(zh_cn: "应用比例", en: "Rate"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        }
    ]);
}
