use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;

pub struct BuffBeidouC6;

impl<A: Attribute> Buff<A> for BuffBeidouC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusElectro, "BUFF：北斗命座「北斗祓幽孽」", 0.15);
    }
}

impl BuffMeta for BuffBeidouC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BeidouC6,
        chs: "北斗-「北斗祓幽孽」",
        image: BuffImage::Avatar(CharacterName::Beidou),
        genre: BuffGenre::Character,
        description: Some("斫雷持续期间，周围敌人的雷元素抗性降低15%。"),
        from: BuffFrom::Character(CharacterName::Beidou),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffBeidouC6)
    }
}
