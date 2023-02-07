use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffKamisatoAyakaC4;

impl<A: Attribute> Buff<A> for BuffKamisatoAyakaC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: 神里绫华四命「盈缺流返」", 0.3);
    }
}

impl BuffMeta for BuffKamisatoAyakaC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KamisatoAyakaC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "神里绫华-「盈缺流返」",
            en: "Ayaka-「Ebb and Flow」",
        ),
        image: BuffImage::Avatar(CharacterName::KamisatoAyaka),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "绫华命座4：敌人受到神里流•霜灭的霜见雪关扉造成的伤害后，防御力降低30%，持续6秒。",
            en: "绫华命座4：敌人受到神里流•霜灭的霜见雪关扉造成的伤害后，防御力降低30%，持续6秒。",
        )),
        from: BuffFrom::Character(CharacterName::KamisatoAyaka),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKamisatoAyakaC4)
    }
}
