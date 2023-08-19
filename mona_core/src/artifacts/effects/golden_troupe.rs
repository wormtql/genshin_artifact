use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct GoldenTroupeEffect {
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for GoldenTroupeEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusElementalSkill, "黄金剧团2", 0.2);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusElementalSkill, "黄金剧团4", 0.25 + self.rate * 0.25);
    }
}

pub struct GoldenTroupe;

impl ArtifactTrait for GoldenTroupe {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GoldenTroupeEffect {
            rate: config.config_golden_troupe.rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::GoldenTroupe,
        name_mona: "GoldenTroupe",
        name_locale: locale!(
            zh_cn: "黄金剧团",
            en: "Golden Troupe"
        ),
        flower: Some(locale!(zh_cn: "黄金乐曲的变奏", en: "Golden Song's Variation")),
        feather: Some(locale!(zh_cn: "黄金飞鸟的落羽", en: "Golden Bird's Shedding")),
        sand: Some(locale!(zh_cn: "黄金时代的先声", en: "Golden Era's Prelude")),
        goblet: Some(locale!(zh_cn: "黄金之夜的喧嚣", en: "Golden Night's Bustle")),
        head: Some(locale!(zh_cn: "黄金剧团的奖赏", en: "Golden Troupe's Reward")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "元素战技造成的伤害提升20%。",
            en: "Increases Elemental Skill DMG by 20%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "元素战技造成的伤害提升25%；此外，处于队伍后台时，元素战技造成的伤害还将进一步提升25%，该效果将在登场后2秒移除。",
            en: "Increases Elemental Skill DMG by 25%. Additionally, when not on the field, Elemental Skill DMG will be further increased by 25%. This effect will be cleared 2s after taking the field."
        )),
        effect5: None,
        internal_id: 15032
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "被动应用比例",
                en: "Effect Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
