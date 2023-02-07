use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffATKPercentage {
    pub value: f64,
}

impl BuffATKPercentage {
    pub fn new(config: &BuffConfig) -> BuffATKPercentage {
        let value = match *config {
            BuffConfig::ATKPercentage { p } => p / 100.0,
            _ => 0.0
        };
        BuffATKPercentage {
            value
        }
    }
}

impl<T: Attribute> Buff<T> for BuffATKPercentage {
    fn change_attribute(&self, attribute: &mut T) {
        attribute.add_atk_percentage("BUFF: 攻击力%", self.value);
    }
}

impl BuffMeta for BuffATKPercentage {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ATKPercentage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "攻击力%",
            en: "ATK%",
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
        Box::new(BuffATKPercentage::new(b))
    }
}
