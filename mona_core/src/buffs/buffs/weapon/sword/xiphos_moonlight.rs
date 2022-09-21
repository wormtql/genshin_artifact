use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffXiphosMoonlight {
    pub em: f64,
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffXiphosMoonlight {
    fn change_attribute(&self, attribute: &mut A) {
        let per = self.refine as f64 * 0.00009 + 0.00027;
        let value = per * self.em * 0.3;

        attribute.set_value_by(AttributeName::Recharge, "BUFF: 西福斯的月光被动", value);
    }
}

impl BuffMeta for BuffXiphosMoonlight {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XiphosMoonlight,
        chs: "",
        image: BuffImage::Weapon(WeaponName::XiphosMoonlight),
        genre: BuffGenre::Weapon,
        description: Some(""),
        from: BuffFrom::Weapon(WeaponName::XiphosMoonlight)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "em",
            title: "w27",
            config: ItemConfigType::FloatInput { default: 900.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (em, refine) = match *b {
            BuffConfig::XiphosMoonlight { em, refine } => (em, refine),
            _ => (0.0, 1)
        };

        Box::new(BuffXiphosMoonlight {
            em, refine
        })
    }
}
