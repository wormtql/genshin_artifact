use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffJeanC4;

impl<A: Attribute> Buff<A> for BuffJeanC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF: 琴四命「蒲公英的国土」", 0.4);
    }
}

impl BuffMeta for BuffJeanC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::JeanC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "琴-「蒲公英的国土」",
            en: "Jean-「Lands of Dandelion」",
        ),
        image: BuffImage::Avatar(CharacterName::Jean),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "琴命座4：在蒲公英之风的领域内，所有敌人的风元素抗性下降40％。",
            en: "琴命座4：在蒲公英之风的领域内，所有敌人的风元素抗性下降40％。",
        )),
        from: BuffFrom::Character(CharacterName::Jean),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffJeanC4)
    }
}