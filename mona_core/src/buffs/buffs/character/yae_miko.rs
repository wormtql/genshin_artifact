use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffYaeMikoC4;

impl<A: Attribute> Buff<A> for BuffYaeMikoC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusElectro, "BUFF: 八重神子四命：「绯樱引雷章」", 0.2);
    }
}

impl BuffMeta for BuffYaeMikoC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YaeMikoC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "八重神子-「绯樱引雷章」",
            en: "Yae-「Sakura Channeling」",
        ),
        image: BuffImage::Avatar(CharacterName::YaeMiko),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "八重神子命座4：杀生樱的落雷命中敌人后，队伍中附近的所有角色获得20%雷元素伤害加成，持续5秒。",
            en: "八重神子命座4：杀生樱的落雷命中敌人后，队伍中附近的所有角色获得20%雷元素伤害加成，持续5秒。",
        )),
        from: BuffFrom::Character(CharacterName::YaeMiko)
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffYaeMikoC4)
    }
}
