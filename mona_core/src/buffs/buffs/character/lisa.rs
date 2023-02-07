use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffLisaTalent2;

impl<A: Attribute> Buff<A> for BuffLisaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: 丽莎天赋「静电场力」", 0.15);
    }
}

impl BuffMeta for BuffLisaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LisaTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "丽莎-「静电场力」",
            en: "Lisa-「Static Electricity Field」",
        ),
        image: BuffImage::Avatar(CharacterName::Lisa),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "丽莎天赋2：敌人受到蔷薇的雷光攻击后，降低15%防御力，持续10秒。",
            en: "丽莎天赋2：敌人受到蔷薇的雷光攻击后，降低15%防御力，持续10秒。",
        )),
        from: BuffFrom::Character(CharacterName::Lisa),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLisaTalent2)
    }
}
