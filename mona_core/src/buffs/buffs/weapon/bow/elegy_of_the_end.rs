use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffElegyOfTheEnd {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffElegyOfTheEnd {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine * 25 + 75;
        attribute.set_value_by(AttributeName::ElementalMastery, "终末嗟叹之诗「千年的大乐章·别离之歌」", v as f64);

        let v = (self.refine * 5 + 15) as f64 / 100.0;
        attribute.add_atk_percentage("终末嗟叹之诗「千年的大乐章·别离之歌」", v);
    }
}

impl BuffMeta for BuffElegyOfTheEnd {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ElegyOfTheEnd,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "终末嗟叹之诗-「千年的大乐章·别离之歌」",
            en: "Elegy for the End-「Millennial Movement: Farewell Song」",
        ),
        image: BuffImage::Weapon(WeaponName::ElegyOfTheEnd),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "千年的大乐章·别离之歌」效果：元素精通提高100/125/150/175/200点，攻击力提升20%/25%/30%/35%/40%。",
            en: "千年的大乐章·别离之歌」效果：元素精通提高100/125/150/175/200点，攻击力提升20%/25%/30%/35%/40%。",
        )),
        from: BuffFrom::Weapon(WeaponName::ElegyOfTheEnd),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::ElegyOfTheEnd { refine } => refine,
            _ => 1
        };

        Box::new(BuffElegyOfTheEnd {
            refine
        })
    }
}
