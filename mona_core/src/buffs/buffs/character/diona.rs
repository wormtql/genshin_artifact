use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffDionaC6G50;

impl<A: Attribute> Buff<A> for BuffDionaC6G50 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 迪奥娜6命「猫尾打烊之时」", 200.0);
    }
}

impl BuffMeta for BuffDionaC6G50 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DionaC6G50,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "迪奥娜-「猫尾打烊之时」",
            en: "Diona-「Cat's Tail Closing Time」",
        ),
        image: BuffImage::Avatar(CharacterName::Diona),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "迪奥娜命座6：生命值高于50%时，元素精通提升200。",
            en: "Diona C6: Elemental Mastery increased by 200 when HP is above 50%.",
        )),
        from: BuffFrom::Character(CharacterName::Diona)
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffDionaC6G50)
    }
}
