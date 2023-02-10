use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::RaidenShogun;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffRaidenShogunE {
    pub skill2: usize,
    pub energy: usize,
}

impl<A: Attribute> Buff<A> for BuffRaidenShogunE {
    fn change_attribute(&self, attribute: &mut A) {
        let p = RaidenShogun::SKILL.elemental_skill_q_bonus[self.skill2 - 1];
        let q_bonus = p * self.energy as f64;
        attribute.set_value_by(AttributeName::BonusElementalBurst, "BUFF: 「雷罚恶曜之眼」", q_bonus);
    }
}

impl BuffMeta for BuffRaidenShogunE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RaidenShogunE,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "雷电将军-「雷罚恶曜之眼」",
            en: "Raiden Shogun-「Eye of Stormy Judgment」",
        ),
        image: BuffImage::Avatar(CharacterName::RaidenShogun),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "雷电将军E技能：雷罚恶曜之眼的角色在持续期间内，元素爆发造成的伤害获得提升，提升程度基于元素爆发的元素能量。",
            en: "雷电将军E技能：雷罚恶曜之眼的角色在持续期间内，元素爆发造成的伤害获得提升，提升程度基于元素爆发的元素能量。",
        )),
        from: BuffFrom::Character(CharacterName::RaidenShogun),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill2",
            title: crate::common::i18n::locale!(
                zh_cn: "雷电将军E技能等级",
                en: "RaidenShogun E Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 }
        },
        ItemConfig {
            name: "energy",
            title: crate::common::i18n::locale!(
                zh_cn: "受BUFF角色最大元素能量",
                en: "Buffed Character's Max Energy",
            ),
            config: ItemConfigType::Int { min: 20, max: 100, default: 80 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (energy, skill2) = match *b {
            BuffConfig::RaidenShogunE { energy, skill2 } => (energy, skill2),
            _ => (0, 1)
        };

        Box::new(BuffRaidenShogunE {
            energy, skill2
        })
    }
}


pub struct BuffRaidenShogunC4;

impl<A: Attribute> Buff<A> for BuffRaidenShogunC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: 雷电将军四命「誓奉常道」", 0.3);
    }
}

impl BuffMeta for BuffRaidenShogunC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RaidenShogunC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "雷电将军-「誓奉常道」",
            en: "Raiden Shogun-「Pledge of Propriety」",
        ),
        image: BuffImage::Avatar(CharacterName::RaidenShogun),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "雷电将军命座4：奥义•梦想真说施加的梦想一心状态结束后，附近的队伍中所有角色（不包括雷电将军自己）的攻击力提升30%，持续10秒。",
            en: "雷电将军命座4：奥义•梦想真说施加的梦想一心状态结束后，附近的队伍中所有角色（不包括雷电将军自己）的攻击力提升30%，持续10秒。",
        )),
        from: BuffFrom::Character(CharacterName::RaidenShogun),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffRaidenShogunC4)
    }
}
