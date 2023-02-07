use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffWolfsGravestone {
    pub refine: usize
}

impl<A: Attribute> Buff<A> for BuffWolfsGravestone {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.1 + 0.3;
        attribute.add_atk_percentage("BUFF: 狼的末路被动", v);
    }
}

impl BuffMeta for BuffWolfsGravestone {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::WolfsGravestone,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "狼的末路-「如狼般狩猎者」",
            en: "Wolf's Gravestone-「Wolfish Tracker」",
        ),
        image: BuffImage::Weapon(WeaponName::WolfsGravestone),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "攻击命中生命值低于30%的敌人时，队伍中所有成员的攻击力提高40%/50%/60%/70%/80%，持续12秒。该效果30秒只能触发一次。",
            en: "攻击命中生命值低于30%的敌人时，队伍中所有成员的攻击力提高40%/50%/60%/70%/80%，持续12秒。该效果30秒只能触发一次。",
        )),
        from: BuffFrom::Weapon(WeaponName::WolfsGravestone),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::WolfsGravestone { refine } => refine,
            _ => 1
        };

        Box::new(BuffWolfsGravestone {
            refine
        })
    }
}
