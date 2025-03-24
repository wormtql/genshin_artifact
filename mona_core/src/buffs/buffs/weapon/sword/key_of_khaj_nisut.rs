use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::i18n::locale;
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
        name_locale: crate::common::i18n::locale!(
            zh_cn: "圣显之钥-「沉入沙海的史诗」",
            en: "Key of Khaj-Nisut-「Sunken Song of the Sands」",
        ),
        image: BuffImage::Weapon(WeaponName::KeyOfKhajNisut),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "将基于装备者生命值上限的0.2%/0.25%/0.3%/0.35%/0.4%，为队伍中附近所有角色提供元素精通提升，持续20秒。",
            en: "The Elemental Mastery of all nearby party members will be increased by 0.2%/0.25%/0.3%/0.35%/0.4% of the equipping character's max HP for 20s.",
        )),
        from: BuffFrom::Weapon(WeaponName::KeyOfKhajNisut)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "hp",
            title: locale!(
                zh_cn: "生命值",
                en: "HP",
            ),
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
