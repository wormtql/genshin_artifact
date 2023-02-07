use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffCriticalDamage {
    pub value: f64,
}

impl<A: Attribute> Buff<A> for BuffCriticalDamage {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalDamageBase, "BUFF: 暴击伤害", self.value);
    }
}

impl BuffMeta for BuffCriticalDamage {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CriticalDamage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "暴击伤害",
            en: "Crit DMG",
        ),
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: None,
        from: BuffFrom::Common
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::BUFFV1P
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let value = match *b {
            BuffConfig::CriticalDamage { p } => p / 100.0,
            _ => 0.0
        };
        Box::new(BuffCriticalDamage {
            value
        })
    }
}
