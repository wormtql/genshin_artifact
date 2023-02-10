use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::i18n::locale;
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
        name_locale: crate::common::i18n::locale!(
            zh_cn: "玛海菈的水色-「沙上楼阁」",
            en: "Makhaira Aquamarine-「Desert Pavilion」",
        ),
        image: BuffImage::Weapon(WeaponName::MakhairaAquamarine),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "「流浪的晚星」与此相同。基于装备者的元素精通的24%/30%/36%/42%/48%，提升该角色的攻击力，并基于该提升的30%为队伍中附近的其他角色提升攻击力，持续12秒，多件同名武器产生的此效果可以叠加。角色处于队伍后台时也能触发效果。",
            en: "The equipping character will gain 24%/30%/36%/42%/48% of their Elemental Mastery as bonus ATK for 12s, with nearby party members gaining 30% of this buff for the same duration. Multiple instances of this weapon can allow this buff to stack. This effect will still trigger even if the character is not on the field.",
        )),
        from: BuffFrom::Weapon(WeaponName::MakhairaAquamarine)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "元素精通",
                en: "EM"
            ),
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
