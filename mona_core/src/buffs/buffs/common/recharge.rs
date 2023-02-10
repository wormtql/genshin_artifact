use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffRecharge {
    pub value: f64
}

impl<A: Attribute> Buff<A> for BuffRecharge {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::Recharge, "BUFF: 元素充能效率", self.value);
    }
}

impl BuffMeta for BuffRecharge {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::Recharge,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素充能效率",
            en: "Energy Recharge",
        ),
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: None,
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "p",
            title: locale!(
                zh_cn: "数值",
                en: "Number",
            ),
            config: ItemConfigType::FloatPercentageInput { default: 20.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let value = match *b {
            BuffConfig::Recharge { p } => p / 100.0,
            _ => 0.0
        };

        Box::new(BuffRecharge {
            value
        })
    }
}
