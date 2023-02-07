use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffKleeC2;

impl<A: Attribute> Buff<A> for BuffKleeC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: 可莉二命「破破弹片」", 0.23);
    }
}

impl BuffMeta for BuffKleeC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KleeC2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "可莉-「破破弹片」",
            en: "Klee-「Explosive Frags」",
        ),
        image: BuffImage::Avatar(CharacterName::Klee),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "可莉命座2：蹦蹦炸弹的诡雷会使敌人防御力降低23％，持续10秒。",
            en: "可莉命座2：蹦蹦炸弹的诡雷会使敌人防御力降低23％，持续10秒。",
        )),
        from: BuffFrom::Character(CharacterName::Klee),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKleeC2)
    }
}

pub struct BuffKleeC6;

impl<A: Attribute> Buff<A> for BuffKleeC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "BUFF: 可莉六命「火力全开」", 0.1);
    }
}

impl BuffMeta for BuffKleeC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KleeC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "可莉-「火力全开」",
            en: "Klee-「Blazing Delight」",
        ),
        image: BuffImage::Avatar(CharacterName::Klee),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "可莉命座6：施放轰轰火花后的25秒内，队伍中所有角色获得10％火元素伤害加成。",
            en: "可莉命座6：施放轰轰火花后的25秒内，队伍中所有角色获得10％火元素伤害加成。",
        )),
        from: BuffFrom::Character(CharacterName::Klee),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKleeC6)
    }
}
