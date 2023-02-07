use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffFreedomSworn {
    pub refine: usize
}

impl<A: Attribute> Buff<A> for BuffFreedomSworn {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.04 + 0.12;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: 苍古自由之誓「千年的大乐章·抗争之歌」", v);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "BUFF: 苍古自由之誓「千年的大乐章·抗争之歌」", v);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "BUFF: 苍古自由之誓「千年的大乐章·抗争之歌」", v);

        let v = self.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("BUFF: 苍古自由之誓「千年的大乐章·抗争之歌」", v);
    }
}

impl BuffMeta for BuffFreedomSworn {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FreedomSworn,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "苍古自由之誓-「千年的大乐章·抗争之歌」",
            en: "Freedom-Sworn-「Millennial Movement: Song of Resistance」",
        ),
        image: BuffImage::Weapon(WeaponName::FreedomSworn),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "「千年的大乐章·抗争之歌」效果：普通攻击、重击、下落攻击造成的伤害提升16%/20%/24%/28%/32%，攻击力提升20%/25%/30%/35%/40%。",
            en: "「千年的大乐章·抗争之歌」效果：普通攻击、重击、下落攻击造成的伤害提升16%/20%/24%/28%/32%，攻击力提升20%/25%/30%/35%/40%。",
        )),
        from: BuffFrom::Weapon(WeaponName::FreedomSworn),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::FreedomSworn { refine } => refine,
            _ => 1
        };

        Box::new(BuffFreedomSworn {
            refine
        })
    }
}
