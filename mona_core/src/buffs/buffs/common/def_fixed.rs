use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffDEFFixed {
    pub value: f64,
}

impl<A: Attribute> Buff<A> for BuffDEFFixed {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DEFFixed, "BUFF: 防御力", self.value);
    }
}

impl BuffMeta for BuffDEFFixed {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DEFFixed,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "防御力",
            en: "DEF",
        ),
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: None,
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::BUFFV1
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let value = match *b {
            BuffConfig::DEFFixed { value } => value,
            _ => 0.0
        };

        Box::new(BuffDEFFixed {
            value
        })
    }
}
