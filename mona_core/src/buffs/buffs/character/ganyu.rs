use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffGanyuTalent2;

impl<A: Attribute> Buff<A> for BuffGanyuTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusCryo, "BUFF: 甘雨天赋「天地交泰」", 0.2);
    }
}

impl BuffMeta for BuffGanyuTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GanyuTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "甘雨-「天地交泰」",
            en: "Ganyu-「Harmony Between Heaven and Earth」",
        ),
        image: BuffImage::Avatar(CharacterName::Ganyu),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "甘雨天赋2：降众天华领域内的队伍中当前场上角色获得20%冰元素伤害加成。",
            en: "甘雨天赋2：降众天华领域内的队伍中当前场上角色获得20%冰元素伤害加成。",
        )),
        from: BuffFrom::Character(CharacterName::Ganyu)
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGanyuTalent2)
    }
}

pub struct BuffGanyuC1;

impl<A: Attribute> Buff<A> for BuffGanyuC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusCryo, "BUFF: 甘雨1命「饮露」", 0.15);
    }
}

impl BuffMeta for BuffGanyuC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GanyuC1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "甘雨-「饮露」",
            en: "Ganyu-「Dew-Drinker」",
        ),
        image: BuffImage::Avatar(CharacterName::Ganyu),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "甘雨命座1：二段蓄力重击的霜华矢或霜华绽发命中敌人时，会使敌人的冰元素抗性降低15%，持续6秒。",
            en: "甘雨命座1：二段蓄力重击的霜华矢或霜华绽发命中敌人时，会使敌人的冰元素抗性降低15%，持续6秒。",
        )),
        from: BuffFrom::Character(CharacterName::Ganyu),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGanyuC1)
    }
}
