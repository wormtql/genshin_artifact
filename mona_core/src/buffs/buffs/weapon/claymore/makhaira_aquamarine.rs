use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffMakhairaAquamarine {
    pub refine: usize,
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffMakhairaAquamarine {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (0.06 * self.refine as f64 + 0.18) * self.em * 0.3;
        attribute.set_value_by(AttributeName::ATKFixed, "BUFF: 玛海菈的水色", value);
    }
}

impl BuffMeta for BuffMakhairaAquamarine {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MakhairaAquamarine,
        chs: "",
        image: BuffImage::Weapon(WeaponName::MakhairaAquamarine),
        genre: BuffGenre::Weapon,
        description: Some(""),
        from: BuffFrom::Weapon(WeaponName::MakhairaAquamarine)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "em",
            title: "w27",
            config: ItemConfigType::FloatInput { default: 900.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (refine, em) = match *b {
            BuffConfig::MakhairaAquamarine { refine, em } => (refine, em),
            _ => (1, 0.0)
        };

        Box::new(BuffMakhairaAquamarine {
            refine, em
        })
    }
}
