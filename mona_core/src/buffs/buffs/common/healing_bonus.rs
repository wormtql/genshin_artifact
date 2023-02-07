use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffHealingBonus {
    pub value: f64,
}

impl<A: Attribute> Buff<A> for BuffHealingBonus {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::HealingBonus, "BUFF: 治疗加成", self.value);
    }
}

impl BuffMeta for BuffHealingBonus {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HealingBonus,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "治疗加成",
            en: "Healing Bonus",
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
            BuffConfig::HealingBonus { p } => p / 100.0,
            _ => 0.0
        };

        Box::new(BuffHealingBonus {
            value
        })
    }
}
