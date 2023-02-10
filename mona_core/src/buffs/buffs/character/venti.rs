use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffVentiC2 {
    pub levitating: bool
}

impl<A: Attribute> Buff<A> for BuffVentiC2 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = 0.12 + if self.levitating { 0.12 } else { 0.0 };
        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF: 温迪二命「眷恋的泠风」", v);
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: 温迪二命「眷恋的泠风」", v);
    }
}

impl BuffMeta for BuffVentiC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-「眷恋的泠风」",
            en: "Venti-「Breeze of Reminiscence」",
        ),
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "温迪命座2：高天之歌会使敌人的风元素抗性与物理抗性降低12％，持续10秒。被高天之歌击飞的敌人在落地前，风元素抗性与物理抗性额外降低12％。",
            en: "温迪命座2：高天之歌会使敌人的风元素抗性与物理抗性降低12％，持续10秒。被高天之歌击飞的敌人在落地前，风元素抗性与物理抗性额外降低12％。",
        )),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "levitating",
            title: crate::common::i18n::locale!(
                zh_cn: "落地前",
                en: "Levitating",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let levitating = match *b {
            BuffConfig::VentiC2 { levitating } => levitating,
            _ => false
        };

        Box::new(BuffVentiC2 {
            levitating
        })
    }
}


pub struct BuffVentiC6 {
    pub element: Element,
    pub is_convert: bool,
}

impl<A: Attribute> Buff<A> for BuffVentiC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF: 温迪六命「抗争的暴风」", 0.2);
        if self.is_convert {
            let name = AttributeName::bonus_name_by_element(self.element);
            attribute.set_value_by(name, "BUFF: 温迪六命「抗争的暴风」", 0.2);
        }
    }
}

impl BuffMeta for BuffVentiC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-「抗争的暴风」",
            en: "Venti-「Storm of Defiance」",
        ),
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "温迪命座6：温迪六命BUFF。受风神之诗伤害的敌人，风元素抗性降低20％。若产生了元素转化，则使转换的元素抗性也降低20％。",
            en: "温迪命座6：温迪六命BUFF。受风神之诗伤害的敌人，风元素抗性降低20％。若产生了元素转化，则使转换的元素抗性也降低20％。",
        )),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "is_convert",
            title: crate::common::i18n::locale!(
                zh_cn: "发生了元素转化",
                en: "Element Transform Occurred",
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "element",
            title: crate::common::i18n::locale!(
                zh_cn: "转化类型",
                en: "Transform Type",
            ),
            config: ItemConfigType::Element4 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (is_convert, element) = match *b {
            BuffConfig::VentiC6 { is_convert, element } => (is_convert, element),
            _ => (false, Element::Electro)
        };

        Box::new(BuffVentiC6 {
            is_convert, element
        })
    }
}
