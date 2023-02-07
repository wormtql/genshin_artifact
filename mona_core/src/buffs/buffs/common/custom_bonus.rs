use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffCustomBonus {
    pub value: f64
}

impl<A: Attribute> Buff<A> for BuffCustomBonus {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase, "BUFF: 伤害加成", self.value);
    }
}

impl BuffMeta for BuffCustomBonus {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CustomBonus,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "伤害加成",
            en: "DMG Bonus",
        ),
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: None,
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::BUFFV1P
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let value = match *b {
            BuffConfig::CustomBonus { p } => p / 100.0,
            _ => 0.0
        };

        Box::new(BuffCustomBonus {
            value
        })
    }
}
