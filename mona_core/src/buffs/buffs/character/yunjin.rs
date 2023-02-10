use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Yunjin;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffYunjinQ {
    pub skill3: usize,
    pub talent2: bool,
    pub ele_count: usize,
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffYunjinQ {
    fn change_attribute(&self, attribute: &mut A) {
        let base = Yunjin::SKILL.elemental_burst_dmg_bonus[self.skill3 - 1];
        let extra = if self.talent2 {
            match self.ele_count {
                1 => 0.025,
                2 => 0.05,
                3 => 0.075,
                _ => 0.115
            }
        } else {
            0.0
        };
        let v = (base + extra) * self.def;

        attribute.set_value_by(AttributeName::ExtraDmgNormalAttack, "BUFF: 云堇「飞云旗阵」", v);
    }
}

impl BuffMeta for BuffYunjinQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YunjinQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "云堇-「飞云旗阵」",
            en: "Yunjin-「Flying Cloud Flag Formation」",
        ),
        image: BuffImage::Avatar(CharacterName::Yunjin),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "云堇Q技能：对敌人造成普通攻击伤害时，基于云堇自己当前的防御力，提高造成的伤害。<br>天赋「莫从恒蹊」：「飞云旗阵」提供的普通攻击伤害提高，当队伍中存在1/2/3/4种元素类型的角色时，数值上进一步追加云堇防御力的2.5%/5.0%/7.5%/11.5%。",
            en: "云堇Q技能：对敌人造成普通攻击伤害时，基于云堇自己当前的防御力，提高造成的伤害。<br>天赋「莫从恒蹊」：「飞云旗阵」提供的普通攻击伤害提高，当队伍中存在1/2/3/4种元素类型的角色时，数值上进一步追加云堇防御力的2.5%/5.0%/7.5%/11.5%。",
        )),
        from: BuffFrom::Character(CharacterName::Yunjin),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: crate::common::i18n::locale!(
                zh_cn: "Q技能等级",
                en: "Q Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
        ItemConfig {
            name: "def",
            title: crate::common::i18n::locale!(
                zh_cn: "云堇的防御力",
                en: "Yunjin DEF",
            ),
            config: ItemConfigType::FloatInput { default: 2000.0 }
        },
        ItemConfig {
            name: "talent2",
            title: crate::common::i18n::locale!(
                zh_cn: "60级突破",
                en: "60 Ascend",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "ele_count",
            title: crate::common::i18n::locale!(
                zh_cn: "队伍不同属性数量",
                en: "Different Element Count",
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (skill3, def, talent2, ele_count) = match *b {
            BuffConfig::YunjinQ { skill3, def, talent2, ele_count } => (skill3, def, talent2, ele_count),
            _ => (1, 0.0, false, 1)
        };

        Box::new(BuffYunjinQ {
            skill3, def, talent2, ele_count
        })
    }
}


pub struct BuffYunjinC2;

impl<A: Attribute> Buff<A> for BuffYunjinC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: 云堇二命「诸般切末」", 0.15);
    }
}

impl BuffMeta for BuffYunjinC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YunjinC2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "云堇-「诸般切末」",
            en: "Yunjin-「Myriad Mise-En-Scène」",
        ),
        image: BuffImage::Avatar(CharacterName::Yunjin),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "云堇命座2：施放破嶂见旌仪后，附近队伍中所有角色普通攻击造成的伤害提高15%，持续12秒。",
            en: "云堇命座2：施放破嶂见旌仪后，附近队伍中所有角色普通攻击造成的伤害提高15%，持续12秒。",
        )),
        from: BuffFrom::Character(CharacterName::Yunjin),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffYunjinC2)
    }
}
