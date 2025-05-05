use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

struct FinaleOfTheDeepGalleriesEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> ArtifactEffect<A> for FinaleOfTheDeepGalleriesEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusCryo, "深廊终曲2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "深廊终曲4", 0.6 * self.rate1);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "深廊终曲4", 0.6 * self.rate2);
    }
}

pub struct FinaleOfTheDeepGalleries;

impl ArtifactTrait for FinaleOfTheDeepGalleries {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(FinaleOfTheDeepGalleriesEffect {
            rate1: config.config_finale_of_the_deep_galleries.rate1,
            rate2: config.config_finale_of_the_deep_galleries.rate2,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::FinaleOfTheDeepGalleries,
        name_mona: "FinaleOfTheDeepGalleries",
        name_locale: locale!(
            zh_cn: "深廊终曲",
            en: "Finale of the Deep Galleries"
        ),
        flower: Some(locale!(
            zh_cn: "深廊的回奏之歌",
            en: "Deep Gallery's Echoing Song"
        )),
        feather: Some(locale!(
            zh_cn: "深廊的漫远之约",
            en: "Deep Gallery's Distant Pact"
        )),
        sand: Some(locale!(
            zh_cn: "深廊的湮落之刻",
            en: "Deep Gallery's Moment of Oblivion"
        )),
        goblet: Some(locale!(
            zh_cn: "深廊的饫赐之宴",
            en: "Deep Gallery's Bestowed Banquet"
        )),
        head: Some(locale!(
            zh_cn: "深廊的遂失之冕",
            en: "Deep Gallery's Lost Crown"
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "获得15%冰元素伤害加成。",
            en: "Cryo DMG Bonus +15%"
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "装备者的元素能量为0时，普通攻击造成的伤害提升60%，元素爆发造成的伤害提升60%。装备者的普通攻击造成伤害后，上述元素爆发伤害提升效果将失效6秒；装备者的元素爆发造成伤害后，上述普通攻击伤害提升效果将失效6秒。角色处于队伍后台也能触发。",
            en: "When the equipping character has 0 Elemental Energy, Normal Attack DMG is increased by 60% and Elemental Burst DMG is increased by 60%. After the equipping character deals Normal Attack DMG, the aforementioned Elemental Burst effect will stop applying for 6s. After the equipping character deals Elemental Burst DMG, the aforementioned Normal Attack effect will stop applying for 6s. This effect can trigger even if the equipping character is off the field."
        )),
        effect5: None,
        internal_id: 15040,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "效果1比例（普通攻击）",
                en: "Effect 1 Rate (Normal ATK)"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果2比例（元素爆发）",
                en: "Effect 2 Rate (Elemental Burst)"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
