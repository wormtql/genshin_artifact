use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffAratakiIttoC4;

impl<A: Attribute> Buff<A> for BuffAratakiIttoC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: 荒泷一斗命座「奉行牢狱，茶饭之所」", 0.2);
        attribute.add_def_percentage("BUFF: 荒泷一斗命座「奉行牢狱，茶饭之所」", 0.2);
    }
}

impl BuffMeta for BuffAratakiIttoC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AratakiIttoC4,
        chs: "荒泷一斗-「奉行牢狱，茶饭之所」",
        image: BuffImage::Avatar(CharacterName::AratakiItto),
        genre: BuffGenre::Character,
        description: Some("荒泷一斗命座4：最恶鬼王•一斗轰临！！施加的「怒目鬼王」状态结束后，附近的队伍中所有角色的防御力提升20%，攻击力提升20%，持续10秒。"),
        from: BuffFrom::Character(CharacterName::AratakiItto),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAratakiIttoC4)
    }
}
