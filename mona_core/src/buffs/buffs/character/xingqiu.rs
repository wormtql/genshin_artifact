use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffXingqiuC2;

impl<A: Attribute> Buff<A> for BuffXingqiuC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusHydro, "BUFF: 行秋二命「天青现虹」", 0.15);
    }
}

impl BuffMeta for BuffXingqiuC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XingqiuC2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "行秋-「天青现虹」",
            en: "Xingqiu-「Rainbow Upon the Azure Sky」",
        ),
        image: BuffImage::Avatar(CharacterName::Xingqiu),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "行秋命座2：受到剑雨攻击的敌人，水元素抗性降低15％，持续4秒。",
            en: "行秋命座2：受到剑雨攻击的敌人，水元素抗性降低15％，持续4秒。",
        )),
        from: BuffFrom::Character(CharacterName::Xingqiu),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXingqiuC2)
    }
}
