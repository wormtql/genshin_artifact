use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffRazorC4;

impl<A: Attribute> Buff<A> for BuffRazorC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: 雷泽四命「撕咬」", 0.15);
    }
}

impl BuffMeta for BuffRazorC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RazorC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "雷泽-「撕咬」",
            en: "Razor-「Bite」",
        ),
        image: BuffImage::Avatar(CharacterName::Razor),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "雷泽命座4：利爪与苍雷点按时，会使命中的敌人防御力降低15％，持续7秒。",
            en: "雷泽命座4：利爪与苍雷点按时，会使命中的敌人防御力降低15％，持续7秒。",
        )),
        from: BuffFrom::Character(CharacterName::Razor),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffRazorC4)
    }
}
