use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffDEFPercentage {
    pub value: f64
}

impl BuffDEFPercentage {
    pub fn new(config: &BuffConfig) -> Self {
        let value = match *config {
            BuffConfig::DEFPercentage { p } => p / 100.0,
            _ => 0.0
        };
        Self {
            value
        }
    }
}

impl<A: Attribute> Buff<A> for BuffDEFPercentage {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_def_percentage("BUFF: 防御力%", self.value);
    }
}

impl BuffMeta for BuffDEFPercentage {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DEFPercentage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "防御力%",
            en: "DEF%",
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
        Box::new(BuffDEFPercentage::new(b))
    }
}
