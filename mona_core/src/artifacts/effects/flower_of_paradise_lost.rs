use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct FlowerOfParadiseLostEffect {
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for FlowerOfParadiseLostEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "乐园遗落之花2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceBloom, "乐园遗落之花4", 0.4 * (1.0 + 0.25 * self.stack));
        attribute.set_value_by(AttributeName::EnhanceBurgeon, "乐园遗落之花4", 0.4 * (1.0 + 0.25 * self.stack));
        attribute.set_value_by(AttributeName::EnhanceHyperbloom, "乐园遗落之花4", 0.4 * (1.0 + 0.25 * self.stack));
    }
}

pub struct FlowerOfParadiseLost;

impl ArtifactTrait for FlowerOfParadiseLost {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(FlowerOfParadiseLostEffect {
            stack: config.config_flower_of_paradise_lost.stack,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::FlowerOfParadiseLost,
        name_mona: "FlowerOfParadiseLost",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "乐园遗落之花",
            en: "Flower of Paradise Lost",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "月女的华彩",
            en: "Moon Maiden's Myriad",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "谢落的筵席",
            en: "Wilting Feast",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "凝结的时刻",
            en: "A Moment Congealed",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "守秘的魔瓶",
            en: "Secret-Keeper's Magic Bottle",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "紫晶的花冠",
            en: "Amethyst Crown",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提高80点。",
            en: "Elemental Mastery +80",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "装备者绽放、超绽放、烈绽放反应造成的伤害提升50%。此外，装备者触发绽放、超绽放、烈绽放时，上述效果带来的加成提升25%，该效果持续10秒，至多叠加4次，每1秒至多触发一次。装备者处于队伍后台时依然能触发该效果。",
            en: "The equipping character's Bloom, Hyperbloom, and Burgeon reaction DMG are increased by 50%. Additionally, when the equipping character triggers Bloom, Hyperbloom, or Burgeon they will gain another 25% bonus to the effect mentioned prior. Each stack of this lasts 10s. Max 4 stacks simultaneously. This effect can only be triggered once per second. The character who equips this can still trigger its effects when not on the field.",
        )),
        effect5: None,
        internal_id: 15028,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: crate::common::i18n::locale!(
                zh_cn: "效果等效层数",
                en: "Equivalent Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 },
        }
    ]);
}