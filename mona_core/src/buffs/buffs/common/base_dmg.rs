use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffBaseDmg {
    pub value: f64,
}

impl<A: Attribute> Buff<A> for BuffBaseDmg {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ExtraDmgBase, "BUFF: 基础伤害", self.value);
    }
}

impl BuffMeta for BuffBaseDmg {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BaseDmg,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "基础伤害",
            en: "Base DMG",
        ),
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "跟攻击力*倍率同一个乘区",
            en: "Flat DMG",
        )),
        from: BuffFrom::Common
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::BUFFV1
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let value = match *b {
            BuffConfig::BaseDmg { value } => value,
            _ => 0.0
        };

        Box::new(BuffBaseDmg {
            value
        })
    }
}
