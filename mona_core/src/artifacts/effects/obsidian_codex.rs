use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct ObsidianCodexEffect {
    pub set2_rate: f64,
    pub set4_rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for ObsidianCodexEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase, "黑曜秘典2", self.set2_rate * 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalBase, "黑曜秘典4", self.set4_rate * 0.4);
    }
}

pub struct ObsidianCodex;

impl ArtifactTrait for ObsidianCodex {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ObsidianCodexEffect {
            set2_rate: config.config_obsidian_codex.set2_rate,
            set4_rate: config.config_obsidian_codex.set4_rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ObsidianCodex,
        name_mona: "ObsidianCodex",
        name_locale: locale!(
            zh_cn: "黑曜秘典",
            en: "Obsidian Codex"
        ),
        flower: Some(locale!(
            zh_cn: "异种的期许",
            en: "Reckoning of the Xenogenic"
        )),
        feather: Some(locale!(
            zh_cn: "灵髓的根脉",
            en: "Root of the Spirit-Marrow"
        )),
        sand: Some(locale!(
            zh_cn: "夜域的迷思",
            en: "Myths of the Night Realm"
        )),
        goblet: Some(locale!(
            zh_cn: "纷争的前宴",
            en: "Pre-Banquet of the Contenders"
        )),
        head: Some(locale!(
            zh_cn: "诸圣的礼冠",
            en: "Crown of the Saints"
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "装备者处于夜魂加持状态，并且在场上时，造成的伤害提高15%。",
            en: "While the equipping character is in Nightsoul's Blessing and is on the field, their DMG dealt is increased by 15%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "装备者在场上消耗1点夜魂值后，暴击率提高40%，持续6秒。该效果每1秒至多触发一次。",
            en: "After the equipping character consumes 1 Nightsoul point while on the field, CRIT Rate increases by 40% for 6s. This effect can trigger once every second."
        )),
        effect5: None,
        internal_id: 15038,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "set4_rate",
            title: locale!(
                zh_cn: "四件套被动比例",
                en: "4-Set Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG2: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "set2_rate",
            title: locale!(
                zh_cn: "二件套被动比例",
                en: "2-Set Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}