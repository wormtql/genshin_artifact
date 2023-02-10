use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Mona;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffMonaQ {
    pub c4: bool,
    pub skill3: usize,
}

impl<A: Attribute> Buff<A> for BuffMonaQ {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = Mona::SKILL.elemental_burst_bonus[self.skill3 - 1];
        attribute.set_value_by(AttributeName::BonusBase, "BUFF: 莫娜「星异」", bonus);
        if self.c4 {
            attribute.set_value_by(AttributeName::CriticalBase, "BUFF: 莫娜四命「灭绝的预言」", 0.15);
        }
    }
}

impl BuffMeta for BuffMonaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "莫娜-「星异」",
            en: "Mona-「Omen」",
        ),
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "莫娜Q技能：对敌人施加星异的伤害加成效果，并以此提高这一次造成的伤害。四命：队伍中所有角色攻击处于星异状态下的敌人时，暴击率提升15%",
            en: "莫娜Q技能：对敌人施加星异的伤害加成效果，并以此提高这一次造成的伤害。四命：队伍中所有角色攻击处于星异状态下的敌人时，暴击率提升15%",
        )),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: crate::common::i18n::locale!(
                zh_cn: "Q技能等级",
                en: "Q Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 9 }
        },
        ItemConfig {
            name: "c4",
            title: crate::common::i18n::locale!(
                zh_cn: "是否4命",
                en: "C4",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (c4, skill3) = match *b {
            BuffConfig::MonaQ { c4, skill3 } => (c4, skill3),
            _ => (false, 1)
        };
        Box::new(BuffMonaQ {
            c4, skill3
        })
    }
}


pub struct BuffMonaC1;

impl<A: Attribute> Buff<A> for BuffMonaC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "BUFF: 莫娜一命「沉没的预言」", 0.15);
        attribute.set_value_by(AttributeName::EnhanceVaporize, "BUFF: 莫娜一命「沉没的预言」", 0.15);
        attribute.set_value_by(AttributeName::EnhanceSwirlHydro, "BUFF: 莫娜一命「沉没的预言」", 0.15);
    }
}

impl BuffMeta for BuffMonaC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaC1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "莫娜-「沉没的预言」",
            en: "Mona-「Prophecy of Submersion」",
        ),
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "莫娜命座1：队伍中自己的角色攻击命中处于星异状态下的敌人后的8秒内，水元素相关反应的效果提升：<br>•感电反应造成的伤害提升15%，蒸发反应造成的伤害提升15%，水元素扩散反应造成的伤害提升15%",
            en: "莫娜命座1：队伍中自己的角色攻击命中处于星异状态下的敌人后的8秒内，水元素相关反应的效果提升：<br>•感电反应造成的伤害提升15%，蒸发反应造成的伤害提升15%，水元素扩散反应造成的伤害提升15%",
        )),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffMonaC1)
    }
}
