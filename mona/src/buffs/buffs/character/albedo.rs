use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffAlbedoTalent2;

impl<A: Attribute> Buff<A> for BuffAlbedoTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 阿贝多天赋「瓶中人的天慧」", 125.0);
    }
}

impl BuffMeta for BuffAlbedoTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AlbedoTalent2,
        chs: "阿贝多-「瓶中人的天慧」",
        image: BuffImage::Avatar(CharacterName::Albedo),
        genre: BuffGenre::Character,
        description: Some("阿贝多天赋2：释放诞生式·大地之潮时,使附近的队伍中角色的元素精通提高125点，持续10秒"),
        from: BuffFrom::Character(CharacterName::Albedo),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAlbedoTalent2)
    }
}
