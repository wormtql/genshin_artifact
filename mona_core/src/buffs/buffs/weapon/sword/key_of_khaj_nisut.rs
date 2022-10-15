use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffKeyOfKhajNisut {
    pub hp: f64,
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffKeyOfKhajNisut {
    fn change_attribute(&self, attribute: &mut A) {
        let value = 0.0005 * self.refine as f64 + 0.0015;
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 圣显之钥被动", value * self.hp);
    }
}

impl BuffMeta for BuffKeyOfKhajNisut {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KeyOfKhajNisut,
        chs: "",
        image: BuffImage::Weapon(WeaponName::KeyOfKhajNisut),
        genre: BuffGenre::Weapon,
        description: Some(""),
        from: BuffFrom::Weapon(WeaponName::KeyOfKhajNisut)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "hp",
            title: "w28",
            config: ItemConfigType::FloatInput { default: 20000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (hp, refine) = match *b {
            BuffConfig::KeyOfKhajNisut { hp, refine } => (hp, refine),
            _ => (0.0, 1)
        };

        Box::new(BuffKeyOfKhajNisut { hp, refine })
    }
}
