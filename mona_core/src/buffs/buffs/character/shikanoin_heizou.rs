use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffShikanoinHeizouTalent2;

impl<A: Attribute> Buff<A> for BuffShikanoinHeizouTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF：因由勘破", 80.0);
    }
}

impl BuffMeta for BuffShikanoinHeizouTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShikanoinHeizouTalent2,
        chs: "鹿野院平藏-因由勘破",
        image: BuffImage::Avatar(CharacterName::ShikanoinHeizou),
        genre: BuffGenre::Character,
        description: Some("鹿野院平藏天赋2：勠心拳命中敌人后，队伍中所有角色（不包括鹿野院平藏自己）的元素精通提升80点，持续10秒。"),
        from: BuffFrom::Common
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffShikanoinHeizouTalent2)
    }
}