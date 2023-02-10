use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffHakushinRing {
    pub refine: usize,
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffHakushinRing {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus_name = AttributeName::bonus_name_by_element(self.element);
        let refine = self.refine as f64;
        attribute.set_value_by(bonus_name, "BUFF: 白辰之环", refine * 0.025 + 0.075);
    }
}

impl BuffMeta for BuffHakushinRing {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HakushinRing,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "白辰之环-「樱之斋宫」",
            en: "Hakushin Ring-「Sakura Saiguu」",
        ),
        image: BuffImage::Weapon(WeaponName::HakushinRing),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "樱之斋宫：装备该武器的角色触发雷元素相关反应后，队伍中附近的与该元素反应相关的元素类型的角色，获得10%/12.5%/15%/17.5%/20%对应元素的元素伤害加成，持续6秒。通过这种方式，角色获得的元素伤害加成无法叠加。",
            en: "樱之斋宫：装备该武器的角色触发雷元素相关反应后，队伍中附近的与该元素反应相关的元素类型的角色，获得10%/12.5%/15%/17.5%/20%对应元素的元素伤害加成，持续6秒。通过这种方式，角色获得的元素伤害加成无法叠加。",
        )),
        from: BuffFrom::Weapon(WeaponName::HakushinRing)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "element",
            title: crate::common::i18n::locale!(
                zh_cn: "元素",
                en: "Element",
            ),
            config: ItemConfigType::Element8 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (refine, element) = match *b {
            BuffConfig::HakushinRing { refine, element } => (refine, element),
            _ => (1, Element::Electro)
        };

        Box::new(BuffHakushinRing {
            refine, element
        })
    }
}
