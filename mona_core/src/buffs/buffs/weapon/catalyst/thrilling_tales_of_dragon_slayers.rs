use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffThrillingTalesOfDragonSlayers {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffThrillingTalesOfDragonSlayers {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.06 + 0.18;
        attribute.add_atk_percentage("BUFF: 讨龙英杰谭被动", v);
    }
}

impl BuffMeta for BuffThrillingTalesOfDragonSlayers {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ThrillingTalesOfDragonSlayers,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "讨龙英杰谭-「传承」",
            en: "Thrilling Tales of Dragon Slayers-「Heritage」",
        ),
        image: BuffImage::Weapon(WeaponName::ThrillingTalesOfDragonSlayers),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "传承：主动切换角色时，新登场的角色攻击力提升24%/30%/36%/42%/48%，持续10秒。该效果每20秒只能触发一次。",
            en: "传承：主动切换角色时，新登场的角色攻击力提升24%/30%/36%/42%/48%，持续10秒。该效果每20秒只能触发一次。",
        )),
        from: BuffFrom::Weapon(WeaponName::ThrillingTalesOfDragonSlayers),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::ThrillingTalesOfDragonSlayers { refine } => refine,
            _ => 1
        };

        Box::new(BuffThrillingTalesOfDragonSlayers {
            refine
        })
    }
}

