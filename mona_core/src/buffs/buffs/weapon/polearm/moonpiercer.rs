use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffMoonpiercer {
    refine: usize
}

impl<A: Attribute> Buff<A> for BuffMoonpiercer {
    fn change_attribute(&self, attribute: &mut A) {
        let value = self.refine as f64 * 0.04 + 0.12;
        attribute.add_atk_percentage("BUFF: 贯月失「幽林月影」", value);
    }
}

impl BuffMeta for BuffMoonpiercer {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::Moonpiercer,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "贯月矢-「苏生之叶」",
            en: "Moonpiercer-「Leaf of Revival」",
        ),
        image: BuffImage::Weapon(WeaponName::Moonpiercer),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "拾取苏生之叶的角色攻击力提升16%/20%/24%/28%/32%",
            en: "When picked up, the Leaf will grant the character 16% ATK for 12s",
        )),
        from: BuffFrom::Weapon(WeaponName::Moonpiercer)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::Moonpiercer { refine } => refine,
            _ => 1
        };

        Box::new(BuffMoonpiercer { refine })
    }
}
