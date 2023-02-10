use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Shenhe;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffShenheE {
    pub skill2: usize,
    pub atk: f64
}

impl<A: Attribute> Buff<A> for BuffShenheE {
    fn change_attribute(&self, attribute: &mut A) {
        let p = Shenhe::SKILL.elemental_skill_damage_bonus[self.skill2 - 1];
        let base = p * self.atk;
        attribute.set_value_by(AttributeName::ExtraDmgCryo, "BUFF: 申鹤「冰翎」", base);
    }
}

impl BuffMeta for BuffShenheE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheE,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "申鹤-「冰翎」",
            en: "Shenhe-「Icy Quill」",
        ),
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "申鹤E技能：基于申鹤自己当前的攻击力，提高造成的伤害。",
            en: "申鹤E技能：基于申鹤自己当前的攻击力，提高造成的伤害。",
        )),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: crate::common::i18n::locale!(
                zh_cn: "申鹤的攻击力",
                en: "Shenhe ATK",
            ),
            config: ItemConfigType::FloatInput { default: 3000.0 }
        },
        ItemConfig {
            name: "skill2",
            title: crate::common::i18n::locale!(
                zh_cn: "申鹤E技能等级",
                en: "Shenhe E Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (atk, skill2) = match *b {
            BuffConfig::ShenheE { atk, skill2 } => (atk, skill2),
            _ => (0.0, 8)
        };

        Box::new(BuffShenheE {
            atk, skill2
        })
    }
}


pub struct BuffShenheQ {
    pub skill3: usize
}

impl<A: Attribute> Buff<A> for BuffShenheQ {
    fn change_attribute(&self, attribute: &mut A) {
        let v = Shenhe::SKILL.elemental_burst_res_minus[self.skill3 - 1];
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: 申鹤「神女遣灵真诀」", v);
        attribute.set_value_by(AttributeName::ResMinusCryo, "BUFF: 申鹤「神女遣灵真诀」", v);
    }
}

impl BuffMeta for BuffShenheQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "申鹤-「神女遣灵真诀」减抗",
            en: "Shenhe-「Divine Maiden's Deliverance」减抗",
        ),
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "申鹤Q技能：「箓灵」将结成领域，使其中敌人的冰元素抗性与物理抗性降低。",
            en: "申鹤Q技能：「箓灵」将结成领域，使其中敌人的冰元素抗性与物理抗性降低。",
        )),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: crate::common::i18n::locale!(
                zh_cn: "申鹤Q技能等级",
                en: "Shenhe Q Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let skill3 = match *b {
            BuffConfig::ShenheQ { skill3 } => skill3,
            _ => 8
        };

        Box::new(BuffShenheQ {
            skill3
        })
    }
}


pub struct BuffShenheTalent1 {
    pub c2: bool
}

impl<A: Attribute> Buff<A> for BuffShenheTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusCryo, "BUFF: 申鹤天赋「大洞弥罗尊法」", 0.15);
        if self.c2 {
            attribute.set_value_by(AttributeName::CriticalDamageCryo, "BUFF: 申鹤天赋「大洞弥罗尊法」", 0.15);
        }
    }
}

impl BuffMeta for BuffShenheTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "申鹤-「大洞弥罗尊法」",
            en: "Shenhe-「Deific Embrace」",
        ),
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "申鹤天赋1：处于神女遣灵真诀的领域中的当前场上角色，冰元素伤害加成提高15%。二命：领域中的当前场上角色，冰元素伤害的暴击伤害提高15%。",
            en: "申鹤天赋1：处于神女遣灵真诀的领域中的当前场上角色，冰元素伤害加成提高15%。二命：领域中的当前场上角色，冰元素伤害的暴击伤害提高15%。",
        )),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2",
            title: crate::common::i18n::locale!(
                zh_cn: "是否2命",
                en: "C2",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let c2 = match *b {
            BuffConfig::ShenheTalent1 { c2 } => c2,
            _ => false,
        };

        Box::new(BuffShenheTalent1 {
            c2
        })
    }
}


pub struct BuffShenheTalent2 {
    pub t: usize
}

impl<A: Attribute> Buff<A> for BuffShenheTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.t == 0 {
            attribute.set_value_by(AttributeName::BonusElementalSkill, "BUFF: 申鹤天赋「缚灵通真法印」", 0.15);
            attribute.set_value_by(AttributeName::BonusElementalBurst, "BUFF: 申鹤天赋「缚灵通真法印」", 0.15);
        } else {
            attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: 申鹤天赋「缚灵通真法印」", 0.15);
            attribute.set_value_by(AttributeName::BonusChargedAttack, "BUFF: 申鹤天赋「缚灵通真法印」", 0.15);
            attribute.set_value_by(AttributeName::BonusPlungingAttack, "BUFF: 申鹤天赋「缚灵通真法印」", 0.15);
        }
    }
}

impl BuffMeta for BuffShenheTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "申鹤-「缚灵通真法印」",
            en: "Shenhe-「Spirit Communion Seal」",
        ),
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "申鹤天赋2：申鹤施放仰灵威召将役咒后，将使附近的队伍中所有角色获得如下效果：<br>•点按：元素战技和元素爆发造成的伤害提高15%，持续10秒；<br>•长按：普通攻击、重击和下落攻击造成的伤害提高15%，持续15秒。",
            en: "申鹤天赋2：申鹤施放仰灵威召将役咒后，将使附近的队伍中所有角色获得如下效果：<br>•点按：元素战技和元素爆发造成的伤害提高15%，持续10秒；<br>•长按：普通攻击、重击和下落攻击造成的伤害提高15%，持续15秒。",
        )),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "t",
            title: crate::common::i18n::locale!(
                zh_cn: "技能释放方式",
                en: "Hold or Press",
            ),
            config: ItemConfigType::Option {
                options: "点按,长按",
                default: 0
            }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let t = match *b {
            BuffConfig::ShenheTalent2 { t } => t,
            _ => 0
        };

        Box::new(BuffShenheTalent2 {
            t
        })
    }
}
