use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Bennett;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffFaruzanQ {
    pub base_atk: usize,
    pub q_level:usize,
    pub rate_q1:f64,
    pub rate_q2:f64,
    pub rate_talent2:f64,
}

impl<A: Attribute> Buff<A> for BuffFaruzanQ {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus_talent3=[0.237,0.255,0.273,0.296,0.314,0.332,0.356,0.379,0.403,0.427,0.45,0.474,0.504,0.533,0.563];
        let bonus_anemo = bonus_talent3[self.q_level - 1];
        // let bonus_anemo = Faruzan::SKILL.elemental_burst_atk_bonus[self.skill3 - 1];

        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF：法露珊-「诡风之祸」", 0.4 * self.rate_q1);
        attribute.set_value_by(AttributeName::BonusAnemo, "BUFF：法露珊-「祈风之赐」", bonus_anemo * self.rate_q2);
        attribute.set_value_by(AttributeName::ExtraDmgAnemo, "BUFF：法露珊-「七窟遗智」", 0.574 * (self.base_atk as f64) * self.rate_talent2);
    }
}

impl BuffMeta for BuffFaruzanQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FaruzanQ,
        chs: "法露珊 -「抟风秘道」",
        image: BuffImage::Avatar(CharacterName::Nahida), //TODO
        genre: BuffGenre::Character,
        description: Some("法露珊Q技能：「诡风之祸」效果：降低敌人的风元素抗性；<br>「祈风之赐」效果：获得风元素伤害加成；<br>处于抟风秘道的「祈风之赐」效果下的角色，对敌人造成风元素伤害时，基于珐露珊基础攻击力的57.4%，提高造成的伤害。此效果将在对敌人造成风元素伤害的0.1秒后清除，每1秒最多触发一次。"),
        from: BuffFrom::Character(CharacterName::Nahida), //TODO
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "base_atk",
            title: "b46",
            config: ItemConfigType::Int { min:0, max: 1000, default: 196+454 },
        },
        ItemConfig {
            name: "q_level",
            title: "b12",
            config: ItemConfigType::Int { min:1, max: 15, default: 10 },
        },
        ItemConfig {
            name: "rate_q1",
            title: "b47",
            config: ItemConfigType::Float { min:0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "rate_q2",
            title: "b48",
            config: ItemConfigType::Float { min:0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "rate_talent2",
            title: "b49",
            config: ItemConfigType::Float { min:0.0, max: 1.0, default: 0.0 },
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (base_atk, q_level, rate_q1, rate_q2, rate_talent2) = match *b {
            BuffConfig::FaruzanQ { base_atk, q_level, rate_q1, rate_q2, rate_talent2 } => (base_atk, q_level, rate_q1, rate_q2, rate_talent2),
            _ => (0, 10, 0.0, 0.0, 0.0)
        };

        Box::new(BuffFaruzanQ {
            base_atk, q_level, rate_q1, rate_q2, rate_talent2
        })
    }
}
