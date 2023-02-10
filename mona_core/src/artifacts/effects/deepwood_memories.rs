use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct DeepwoodMemoriesEffect {
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for DeepwoodMemoriesEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusDendro, "深林的记忆2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusDendro, "深林的记忆4", 0.3 * self.rate);
    }
}

pub struct DeepwoodMemories;

impl ArtifactTrait for DeepwoodMemories {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DeepwoodMemoriesEffect {
            rate: config.config_deepwood_memories.rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DeepwoodMemories,
        name_mona: "DeepwoodMemories",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "深林的记忆",
            en: "Deepwood Memories",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "迷宫的游人",
            en: "Labyrinth Wayfarer",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "翠蔓的智者",
            en: "Scholar of Vines",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "贤智的定期",
            en: "A Time of Insight",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "迷误者之灯",
            en: "Lamp of the Lost",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "月桂的宝冠",
            en: "Laurel Coronet",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%草元素伤害加成。",
            en: "Dendro DMG Bonus +15%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技或元素爆发命中敌人后，使命中目标的草元素抗性降低30%，持续8秒。装备者处于队伍后台时，依然能触发该效果。",
            en: "After Elemental Skills or Bursts hit opponents, the targets’ Dendro RES will be decreased by 30% for 8s. This effect can be triggered even if the equipping character is not on the field.",
        )),
        effect5: None,
        internal_id: 15025,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}