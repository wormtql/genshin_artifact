use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct SongOfDaysPastEffect {
    pub rate: f64,
    pub regeneration: f64,
}

impl<A: Attribute> ArtifactEffect<A> for SongOfDaysPastEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::HealingBonus, "昔时之歌2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        let amount = self.regeneration as f64 * 0.08 * self.rate;
        attribute.set_value_by(AttributeName::ExtraDmgBase, "昔时之歌4", amount);
    }
}

pub struct SongOfDaysPast;

impl ArtifactTrait for SongOfDaysPast {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(SongOfDaysPastEffect {
            regeneration: config.config_song_of_days_past.regeneration,
            rate: config.config_song_of_days_past.rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::SongOfDaysPast,
        name_mona: "SongOfDaysPast",
        name_locale: locale!(
            zh_cn: "昔时之歌",
            en: "Song of Days Past"
        ),
        flower: Some(locale!(
            zh_cn: "昔时遗落之誓",
            en: "Forgotten Oath of Days Past"
        )),
        feather: Some(locale!(
            zh_cn: "昔时浮想之思",
            en: "Recollection of Days Past"
        )),
        sand: Some(locale!(
            zh_cn: "昔时回映之音",
            en: "Echoing Sound From Days Past"
        )),
        goblet: Some(locale!(
            zh_cn: "昔时应许之梦",
            en: "Promised Dream of Days Past"
        )),
        head: Some(locale!(
            zh_cn: "昔时传奏之诗",
            en: "Poetry of Days Past"
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "治疗加成提高15%。",
            en: "Healing Bonus +15%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "装备者对队伍中的角色进行治疗时，将产生持续6秒的渴盼效果，记录治疗的生命值回复量（包括溢出值）。持续时间结束时，渴盼效果将转变为「彼时的浪潮」效果：队伍中自己的当前场上角色的普通攻击、重击、下落攻击、元素战技与元素爆发命中敌人时，将基于渴盼效果所记录的回复量的8%提高造成的伤害，「彼时的浪潮」将在生效5次或10秒后移除。一次渴盼效果至多记录15000点回复量，同时至多存在一个，能够记录多个装备者的产生的回复量；装备者处于队伍后台时，依然能触发该效果。",
            en: "When the equipping character heals a party member, the Yearning effect will be created for 6s, which records the total amount of healing provided (including overflow healing). When the duration expires, the Yearning effect will be transformed into the \"Waves of Days Past\" effect: When your active party member hits an opponent with a Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, or Elemental Burst, the DMG dealt will be increased by 8% of the total healing amount recorded by the Yearning effect. The \"Waves of Days Past\" effect is removed after it has taken effect 5 times or after 10s. A single instance of the Yearning effect can record up to 15,000 healing, and only a single instance can exist at once, but it can record the healing from multiple equipping characters. Equipping characters on standby can still trigger this effect."
        )),
        effect5: None,
        internal_id: 15033
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "regeneration",
            title: locale!(
                zh_cn: "生命值回复量",
                en: "HP Regeneration"
            ),
            config: ItemConfigType::Int { min: 0, max: 15000, default: 0 },
        },
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "被动应用比例",
                en: "Average Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
