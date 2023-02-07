use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffSongOfBrokenPines {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffSongOfBrokenPines {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("BUFF: 松籁响起之时「千年的大乐章·揭旗之歌」", v);

        let v = self.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "BUFF: 松籁响起之时「千年的大乐章·揭旗之歌」", v);
    }
}

impl BuffMeta for BuffSongOfBrokenPines {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SongOfBrokenPines,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "松籁响起之时-「千年的大乐章·揭旗之歌」",
            en: "Song of Broken Pines-「Millennial Movement: Banner-Hymn」",
        ),
        image: BuffImage::Weapon(WeaponName::SongOfBrokenPines),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "「千年的大乐章·揭旗之歌」效果：普通攻击速度提升12%/15%/18%/21%/24%，攻击力提升20%/25%/30%/35%/40%。",
            en: "「千年的大乐章·揭旗之歌」效果：普通攻击速度提升12%/15%/18%/21%/24%，攻击力提升20%/25%/30%/35%/40%。",
        )),
        from: BuffFrom::Weapon(WeaponName::SongOfBrokenPines),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::SongOfBrokenPines { refine } => refine,
            _ => 1
        };

        Box::new(BuffSongOfBrokenPines {
            refine
        })
    }
}
