use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffXianglingTalent2;

impl<A: Attribute> Buff<A> for BuffXianglingTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: 香菱天赋「绝云朝天椒」", 0.1);
    }
}

impl BuffMeta for BuffXianglingTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianglingTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "香菱-「绝云朝天椒」",
            en: "Xiangling-「Beware, It's Super Hot!」",
        ),
        image: BuffImage::Avatar(CharacterName::Xiangling),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "香菱天赋2：锅巴出击效果结束时，锅巴会在消失的位置留下辣椒。拾取辣椒会提高10%攻击力，持续10秒。",
            en: "香菱天赋2：锅巴出击效果结束时，锅巴会在消失的位置留下辣椒。拾取辣椒会提高10%攻击力，持续10秒。",
        )),
        from: BuffFrom::Character(CharacterName::Xiangling),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXianglingTalent2)
    }
}


pub struct BuffXianglingC1;

impl<A: Attribute> Buff<A> for BuffXianglingC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPyro, "BUFF: 香菱一命「外酥里嫩」", 0.15);
    }
}

impl BuffMeta for BuffXianglingC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianglingC1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "香菱-「外酥里嫩」",
            en: "Xiangling-「Crispy Outside, Tender Inside」",
        ),
        image: BuffImage::Avatar(CharacterName::Xiangling),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "香菱命座1：受到锅巴攻击的敌人，火元素抗性降低15％，持续6秒。",
            en: "香菱命座1：受到锅巴攻击的敌人，火元素抗性降低15％，持续6秒。",
        )),
        from: BuffFrom::Character(CharacterName::Xiangling),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXianglingC1)
    }
}


pub struct BuffXianglingC6;

impl<A: Attribute> Buff<A> for BuffXianglingC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "BUFF: 香菱六命「大龙卷旋火轮」", 0.15);
    }
}

impl BuffMeta for BuffXianglingC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianglingC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "香菱-「大龙卷旋火轮」",
            en: "Xiangling-「Condensed Pyronado」",
        ),
        image: BuffImage::Avatar(CharacterName::Xiangling),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "香菱命座6；旋火轮持续期间，队伍中所有角色获得15％火元素伤害加成。",
            en: "香菱命座6；旋火轮持续期间，队伍中所有角色获得15％火元素伤害加成。",
        )),
        from: BuffFrom::Character(CharacterName::Xiangling),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXianglingC6)
    }
}