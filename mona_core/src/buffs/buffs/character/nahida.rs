use std::cmp::max;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffNahidaTalent1 {
    pub max_em: f64,
}

impl<A: Attribute> Buff<A> for BuffNahidaTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.max_em * 0.25).min(250.0);
        attribute.set_value_by(AttributeName::ElementalMasteryExtra, "BUFF: 纳西妲天赋1", value);
    }
}

impl BuffMeta for BuffNahidaTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NahidaTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纳西妲-「净善摄受明论」",
            en: "Nahida-「Compassion Illuminated」",
        ),
        image: BuffImage::Avatar(CharacterName::Nahida),
        genre: BuffGenre::Character,
        description: None,
        from: BuffFrom::Character(CharacterName::Nahida)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "max_em",
            title: "b44",
            config: ItemConfigType::Float { min: 0.0, max: 3000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let max_em = match *b {
            BuffConfig::NahidaTalent1 { max_em } => max_em,
            _ => 0.0
        };
        Box::new(BuffNahidaTalent1 {
            max_em
        })
    }
}
