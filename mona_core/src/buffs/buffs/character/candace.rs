use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffCandaceQ;

impl<A: Attribute> Buff<A> for BuffCandaceQ {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAndElemental, "BUFF: 坎蒂丝Q", 0.2);
    }
}

impl BuffMeta for BuffCandaceQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CandaceQ,
        chs: "",
        image: BuffImage::Avatar(CharacterName::Candace),
        genre: BuffGenre::Character,
        description: Some(""),
        from: BuffFrom::Character(CharacterName::Candace)
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffCandaceQ)
    }
}

pub struct BuffCandaceTalent2 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffCandaceTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAndElemental, "BUFF: 坎蒂丝天赋2", (self.hp / 1000.0).floor() * 0.005);
    }
}

impl BuffMeta for BuffCandaceTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CandaceTalent2,
        chs: "",
        image: BuffImage::Avatar(CharacterName::Candace),
        genre: BuffGenre::Character,
        description: Some(""),
        from: BuffFrom::Character(CharacterName::Candace)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: "b43",
            config: ItemConfigType::FloatInput { default: 30000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::CandaceTalent2 { hp } => hp,
            _ => 0.0
        };

        Box::new(BuffCandaceTalent2 { hp })
    }
}
