use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::anemo::faruzan::FARUZAN_SKILL;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffFaruzanQ {
    pub base_atk: usize,
    pub q_level: usize,
    pub rate_q1: f64,
    pub rate_q2: f64,
    pub rate_talent2: f64,
    pub enable_c6: bool,
}

impl<A: Attribute> Buff<A> for BuffFaruzanQ {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus_talent3 = FARUZAN_SKILL.q_bonus;
        let bonus_anemo = bonus_talent3[self.q_level - 1];
        // let bonus_anemo = Faruzan::SKILL.elemental_burst_atk_bonus[self.skill3 - 1];

        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF：珐露珊 -「诡风之祸」", 0.3 * self.rate_q1);
        attribute.set_value_by(AttributeName::BonusAnemo, "BUFF：珐露珊 -「祈风之赐」", bonus_anemo * self.rate_q2);
        attribute.set_value_by(AttributeName::ExtraDmgAnemo, "BUFF：珐露珊 -「七窟遗智」", 0.32 * (self.base_atk as f64) * self.rate_talent2);
        if self.enable_c6 {
            attribute.set_value_by(AttributeName::CriticalDamageAnemo, "BUFF：珐露珊-「祈风之赐」", 0.4 * self.rate_q2);
        }
    }
}

impl BuffMeta for BuffFaruzanQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FaruzanQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "法露珊 - 「抟风秘道」",
            en: "Faruzan-「The Wind’s Secret Ways」",
        ),
        image: BuffImage::Avatar(CharacterName::Faruzan),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "珐露珊Q技能：「诡风之祸」效果：降低敌人的风元素抗性；<br>「祈风之赐」效果：获得风元素伤害加成，六命：造成风元素伤害时的暴击伤害提升40%；<br>处于抟风秘道的「祈风之赐」效果下的角色，对敌人造成风元素伤害时，基于珐露珊基础攻击力的32%，提高造成的伤害。此效果将在对敌人造成风元素伤害的0.1秒后清除，每0.8秒最多触发一次。",
            en: "Faruzan Elemental Burst: <br>·When the Whirlwind Pulse hits opponents, it will apply Perfidious Wind's Ruin to them, decreasing their Anemo RES.<br>·The Whirlwind Pulse will also apply Prayerful Wind's Gift to all nearby characters when it is unleashed, granting them Anemo DMG Bonus.<br>When characters affected by The Wind's Secret Ways' Prayerful Wind's Gift deal Anemo DMG to opponents, this DMG will be increased based on 32% of Faruzan's own ATK. This DMG Bonus will be cleared 0.1s after dealing Anemo DMG to opponents, and can be triggered once every 0.8s.",
        )),
        from: BuffFrom::Character(CharacterName::Faruzan),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "base_atk",
            title: crate::common::i18n::locale!(
                zh_cn: "法露珊基础攻击力",
                en: "Faruzan Base ATK",
            ),
            config: ItemConfigType::Int { min: 0, max: 1000, default: 196 + 454 },
        },
        ItemConfig {
            name: "q_level",
            title: crate::common::i18n::locale!(
                zh_cn: "Q技能等级",
                en: "Q Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 },
        },
        ItemConfig {
            name: "rate_q1",
            title: crate::common::i18n::locale!(
                zh_cn: "「诡风之祸」比例",
                en: "Perfidious Wind's Ruin Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "rate_q2",
            title: crate::common::i18n::locale!(
                zh_cn: "「祈风之赐」比例",
                en: "Prayerful Wind's GiftRate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "rate_talent2",
            title: crate::common::i18n::locale!(
                zh_cn: "「七窟遗智」比例（加成hit数/一轮hit总数）",
                en: "「Lost Wisdom of the Seven Caverns Rate（buffed hits count/total hit counts within a cycle）",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "enable_c6",
            title: crate::common::i18n::locale!(
                zh_cn: "六命",
                en: "C6",
            ),
            config: ItemConfigType::Bool { default: false },
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (base_atk, q_level, rate_q1, rate_q2, rate_talent2, enable_c6) = match *b {
            BuffConfig::FaruzanQ { base_atk, q_level, rate_q1, rate_q2, rate_talent2, enable_c6 } => (base_atk, q_level, rate_q1, rate_q2, rate_talent2, enable_c6),
            _ => (0, 10, 0.0, 0.0, 0.0, false)
        };

        Box::new(BuffFaruzanQ {
            base_atk,
            q_level,
            rate_q1,
            rate_q2,
            rate_talent2,
            enable_c6,
        })
    }
}
