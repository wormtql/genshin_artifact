use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffHuTaoTalent1;

impl<A: Attribute> Buff<A> for BuffHuTaoTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalBase, "BUFF: 胡桃天赋「蝶隐之时」", 0.12);
    }
}

impl BuffMeta for BuffHuTaoTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HuTaoTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "胡桃-「蝶隐之时」",
            en: "Hutao-「Flutter By」",
        ),
        image: BuffImage::Avatar(CharacterName::HuTao),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "胡桃天赋1：蝶引来生施加的彼岸蝶舞状态结束后，队伍中所有角色（不包括胡桃自己）的暴击率提高12%，持续8秒。",
            en: "Hu Tao Talent1: When a Paramita Papilio state activated by Guide to Afterlife ends, all allies in the party (excluding Hu Tao herself) will have their CRIT Rate increased by 12% for 8s.",
        )),
        from: BuffFrom::Character(CharacterName::HuTao),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffHuTaoTalent1)
    }
}