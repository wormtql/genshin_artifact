use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffNilouTalent1;

impl<A: Attribute> Buff<A> for BuffNilouTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 「金杯的丰馈」", 100.0);
    }
}

impl BuffMeta for BuffNilouTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NilouTalent1,
        chs: "",
        image: BuffImage::Avatar(CharacterName::Nilou),
        genre: BuffGenre::Character,
        description: Some(""),
        from: BuffFrom::Character(CharacterName::Nilou)
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffNilouTalent1)
    }
}

pub struct BuffNilouTalent2 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffNilouTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.hp >= 31000.0 {
            let value = (((self.hp - 30000.0) / 1000.0).floor() * 0.09).min(4.0);
            attribute.set_value_by(AttributeName::EnhanceBloom, "BUFF: 妮露天赋「翩舞永世之梦」", value);
        }
    }
}

impl BuffMeta for BuffNilouTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NilouTalent2,
        chs: "",
        image: BuffImage::Avatar(CharacterName::Nilou),
        genre: BuffGenre::Character,
        description: Some(""),
        from: BuffFrom::Character(CharacterName::Nilou)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: "b42",
            config: ItemConfigType::FloatInput { default: 60000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::NilouTalent2 { hp } => hp,
            _ => 0.0
        };
        Box::new(BuffNilouTalent2 { hp })
    }
}
