use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffAloyTalent1;

impl<A: Attribute> Buff<A> for BuffAloyTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: 埃洛伊天赋「战斗覆盖」", 0.08);
    }
}

impl BuffMeta for BuffAloyTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AloyTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "埃洛伊-「战斗覆盖」",
            en: "Aloy-「Combat Override」",
        ),
        image: BuffImage::Avatar(CharacterName::Aloy),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "埃洛伊天赋1：埃洛伊获得冰尘雪野的线圈效果时，队伍中附近的其他角色的攻击力提升8%，持续10秒。",
            en: "Aloy Talent1: When Aloy receives the Coil effect from Frozen Wilds, nearby party members' ATK is increased by 8%. This effect lasts 10s.",
        )),
        from: BuffFrom::Character(CharacterName::Aloy),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAloyTalent1)
    }
}
