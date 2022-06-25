use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffCustomBonus {
    pub value: f64,
	pub element: Element,
}

impl<A: Attribute> Buff<A> for BuffCustomBonus {
    fn change_attribute(&self, attribute: &mut A) {
		let name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(name, "BUFF: 伤害加成", self.value);
    }
}

impl BuffMeta for BuffCustomBonus {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CustomBonus,
        chs: "伤害加成",
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: None,
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
	    ItemConfig {
            name: "element",
            title: "伤害加成元素",
            config: ItemConfigType::Element8 { default: Element::Electro }
        },
        ItemConfig::BUFFV1P
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (value, element) = match *b {
            BuffConfig::CustomBonus { p, element } => (p / 100.0, element),
            _ => (0.0, Element::Electro)
        };

        Box::new(BuffCustomBonus {
            value, element
        })
    }
}
