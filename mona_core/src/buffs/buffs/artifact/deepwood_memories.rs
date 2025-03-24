use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffDeepwoodMemories4 {
    pub rate: f64,
}

impl<A: Attribute> Buff<A> for BuffDeepwoodMemories4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusDendro, "BUFF: 深林的记忆4", self.rate * 0.3);
    }
}

impl BuffMeta for BuffDeepwoodMemories4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DeepwoodMemories4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "深林的记忆4",
            en: "DeepwoodMemories4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::DeepwoodMemories),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技或元素爆发命中敌人后，使命中目标的草元素抗性降低30%，持续8秒。装备者处于队伍后台时，依然能触发该效果。",
            en: "After Elemental Skills or Bursts hit opponents, the targets' Dendro RES will be decreased by 30% for 8s. This effect can be triggered even if the equipping character is not on the field.",
        )),
        from: BuffFrom::Artifact(ArtifactSetName::DeepwoodMemories)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "应用比例",
                en: "Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let rate = match *b {
            BuffConfig::DeepwoodMemories4 { rate } => rate,
            _ => 0.0
        };
        Box::new(BuffDeepwoodMemories4 { rate })
    }
}