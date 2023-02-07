use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffHPPercentage {
    pub value: f64
}

impl<A: Attribute> Buff<A> for BuffHPPercentage {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_hp_percentage("BUFF: 生命值%", self.value);
    }
}

impl BuffMeta for BuffHPPercentage {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HPPercentage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "生命值%",
            en: "HP%",
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
            BuffConfig::HPPercentage { p } => p / 100.0,
            _ => 0.0
        };
        Box::new(BuffHPPercentage {
            value
        })
    }
}
