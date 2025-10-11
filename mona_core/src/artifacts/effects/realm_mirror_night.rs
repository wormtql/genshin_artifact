use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::{ArtifactSetName, effect_config::ArtifactEffectConfig};
use crate::artifacts::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct RealmMirrorNightEffect {
    pub moon_state: u8,
    pub moon_reaction_bonus: u8,
}

impl<A: Attribute> ArtifactEffect<A> for RealmMirrorNightEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "穹境示现之夜2", 80.0);
    }
    fn effect4(&self, attribute: &mut A) {
        // 月兆为初辉/满辉时，暴击率提升15%/30%
        let crit = match self.moon_state {
            1 => 0.15,
            2 => 0.30,
            _ => 0.0,
        };
    attribute.set_value_by(AttributeName::CriticalBase, "穹境示现之夜4", crit);
    // 月曜反应增伤
    let bonus = 0.1 * (self.moon_reaction_bonus as f64);
    attribute.set_value_by(AttributeName::EnhanceMoonReaction, "穹境示现之夜4", bonus);
    }
}

pub struct RealmMirrorNight;

impl ArtifactTrait for RealmMirrorNight {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(RealmMirrorNightEffect {
            moon_state: config.config_realm_mirror_night.moon_state,
            moon_reaction_bonus: config.config_realm_mirror_night.moon_reaction_bonus,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::RealmMirrorNight,
        name_mona: "RealmMirrorNight",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "穹境示现之夜",
            en: "Realm Mirror Night",
        ),
        flower: Some(crate::common::i18n::locale!(zh_cn: "渴真之花", en: "Flower of True Thirst")),
        feather: Some(crate::common::i18n::locale!(zh_cn: "深罪之羽", en: "Feather of Deep Sin")),
        sand: Some(crate::common::i18n::locale!(zh_cn: "谕告之钟", en: "Bell of Revelation")),
        goblet: Some(crate::common::i18n::locale!(zh_cn: "满溢之壶", en: "Overflowing Vessel")),
        head: Some(crate::common::i18n::locale!(zh_cn: "永劫之冕", en: "Crown of Eternity")),
        star: (5, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提高80点。",
            en: "Elemental Mastery +80",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "队伍中附近的角色触发月曜反应时，若装备者在场上，将获得持续4秒的「月辉明光·蓄念」效果：队伍的月兆为初辉/满辉时，暴击率提升15%/30%。队伍中的角色每拥有一种不同的「月辉明光」效果，队伍中的所有角色触发的月曜反应造成的伤害提升10%。由「月辉明光」产生的效果无法叠加。",
            en: "When a nearby character triggers a Moonlight Reaction, if the wearer is on the field, gain 'Moonlight Accumulation' for 4s: If the team's Moon Phase is New/Full, CRIT Rate increases by 15%/30%. For each different 'Moonlight Faith' effect, all characters' Moonlight Reaction DMG increases by 10%. Effects from 'Moonlight Faith' cannot stack.",
        )),
        effect5: None,
        internal_id: 20002,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "moon_state",
            title: crate::common::i18n::locale!(zh_cn: "月兆状态", en: "Moon State"),
            config: ItemConfigType::Int { min: 0, max: 2, default: 0 },
        },
        ItemConfig {
            name: "moon_reaction_bonus",
            title: crate::common::i18n::locale!(zh_cn: "月曜反应增伤", en: "Moon Reaction Bonus"),
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 },
        }
    ]);
}
