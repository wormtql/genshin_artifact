use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffZhongliShield;

impl<A: Attribute> Buff<A> for BuffZhongliShield {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusBase, "BUFF: 钟离「玉璋护盾」", 0.2);
    }
}

impl BuffMeta for BuffZhongliShield {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ZhongliShield,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "钟离-「玉璋护盾」",
            en: "Zhongli-「Jade Shield」",
        ),
        image: BuffImage::Avatar(CharacterName::Zhongli),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "钟离盾：使附近小范围内敌人的所有元素抗性与物理抗性降低20%",
            en: "钟离盾：使附近小范围内敌人的所有元素抗性与物理抗性降低20%",
        )),
        from: BuffFrom::Character(CharacterName::Zhongli),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffZhongliShield)
    }
}
