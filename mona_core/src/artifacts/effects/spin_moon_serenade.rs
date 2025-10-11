use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::{ArtifactSetName, effect_config::ArtifactEffectConfig};
use crate::artifacts::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct SpinMoonSerenadeEffect {
    pub moon_state: u8,
    pub moon_reaction_bonus: u8,
}

impl<A: Attribute> ArtifactEffect<A> for SpinMoonSerenadeEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::Recharge, "纺月的夜歌2", 0.2);
    }
    fn effect4(&self, attribute: &mut A) {
        // 月兆为初辉/满辉时，元素精通提高60/120
        let em = match self.moon_state {
            1 => 60.0,
            2 => 120.0,
            _ => 0.0,
        };
    attribute.set_value_by(AttributeName::ElementalMastery, "纺月的夜歌4", em);
    // 月曜反应增伤
    let bonus = 0.1 * (self.moon_reaction_bonus as f64);
    attribute.set_value_by(AttributeName::EnhanceMoonReaction, "纺月的夜歌4", bonus);
    }
}

pub struct SpinMoonSerenade;

impl ArtifactTrait for SpinMoonSerenade {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(SpinMoonSerenadeEffect {
            moon_state: config.config_spin_moon_serenade.moon_state,
            moon_reaction_bonus: config.config_spin_moon_serenade.moon_reaction_bonus,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::SpinMoonSerenade,
        name_mona: "SpinMoonSerenade",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纺月的夜歌",
            en: "Spin Moon Serenade",
        ),
        flower: Some(crate::common::i18n::locale!(zh_cn: "流离者的晶泪", en: "Wanderer's Crystal Tear")),
        feather: Some(crate::common::i18n::locale!(zh_cn: "受福者的白羽", en: "Blessed White Feather")),
        sand: Some(crate::common::i18n::locale!(zh_cn: "祭霜者的迷狂", en: "Frost Worshipper's Frenzy")),
        goblet: Some(crate::common::i18n::locale!(zh_cn: "至纯者的欢荣", en: "Pure One's Joy")),
        head: Some(crate::common::i18n::locale!(zh_cn: "司信者的圣冕", en: "Herald's Crown")),
        star: (5, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素充能效率提高20%。",
            en: "Energy Recharge +20%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "造成元素伤害时，获得持续8秒的「月辉明光·崇信」效果：队伍的月兆为初辉/满辉时，队伍中的所有角色的元素精通提高60点/120点。装备者处于后台时也能触发上述效果。队伍中的角色每拥有一种不同的「月辉明光」效果，队伍中的所有角色触发的月曜反应造成的伤害提升10%。由「月辉明光」产生的效果无法叠加。",
            en: "When dealing Elemental DMG, gain 'Moonlight Faith' for 8s: If the team's Moon Phase is New/Full, all characters gain 60/120 Elemental Mastery. This can be triggered off-field. For each different 'Moonlight Faith' effect, all characters' Moonlight Reaction DMG increases by 10%. Effects from 'Moonlight Faith' cannot stack.",
        )),
        effect5: None,
        internal_id: 20001,
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
