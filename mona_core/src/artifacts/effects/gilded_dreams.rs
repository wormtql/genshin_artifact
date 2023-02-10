use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct GildedDreamsEffect {
    pub same_count: usize,
    pub diff_count: usize,
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for GildedDreamsEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "饰金之梦2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        if self.same_count > 0 {
            attribute.add_atk_percentage("饰金之梦4", (self.same_count.min(3) as f64 * 0.14) * self.rate);
        }
        if self.diff_count > 0 {
            attribute.set_value_by(AttributeName::ElementalMastery, "饰金之梦4", (self.diff_count.min(3) as f64 * 50.0) * self.rate);
        }
    }
}

pub struct GildedDreams;

impl ArtifactTrait for GildedDreams {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GildedDreamsEffect {
            same_count: config.config_gilded_dreams.same_count,
            diff_count: config.config_gilded_dreams.diff_count,
            rate: config.config_gilded_dreams.rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::GildedDreams,
        name_mona: "GildedDreams",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "饰金之梦",
            en: "Gilded Dreams",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "梦中的铁花",
            en: "Dreaming Steelbloom",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "裁断的翎羽",
            en: "Feather of Judgment",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "沉金的岁月",
            en: "The Sunken Years",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "如蜜的终宴",
            en: "Honeyed Final Feast",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "沙王的投影",
            en: "Shadow of the Sand King",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提高80点。",
            en: "Elemental Mastery +80.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "触发元素反应后的8秒内，会根据队伍内其他角色的元素类型，使装备者获得强化：队伍中每存在1个和装备者同类元素的角色，攻击力提升14%；每存在1个和装备者不同元素类型的角色，元素精通提升50点。上述每类效果至多计算3个角色。该效果每8秒至多触发一次。装备者处于队伍后台时，依然能触发该效果。",
            en: "Within 8s of triggering an Elemental Reaction, the character equipping this will obtain buffs based on the Elemental Type of the other party members. ATK is increased by 14% for each party member whose Elemental Type is the same as the equipping character, and Elemental Mastery is increased by 50 for every party member with a different Elemental Type. Each of the aforementioned buffs will count up to 3 characters. This effect can be triggered once every 8s. The character who equips this can still trigger its effects when not on the field.",
        )),
        effect5: None,
        internal_id: 15026,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "same_count",
            title: crate::common::i18n::locale!(
                zh_cn: "同元素角色数",
                en: "Same Element Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "diff_count",
            title: crate::common::i18n::locale!(
                zh_cn: "不同元素角色数",
                en: "Different Element Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
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