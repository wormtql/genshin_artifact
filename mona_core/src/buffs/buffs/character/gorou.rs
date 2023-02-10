use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Gorou;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffGorouE1 {
    pub skill2: usize,
}

impl<A: Attribute> Buff<A> for BuffGorouE1 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = Gorou::SKILL.elemental_skill_def_bonus[self.skill2 - 1];
        attribute.set_value_by(AttributeName::DEFFixed, "BUFF: 五郎-「大将旗指物」-1", bonus);
    }
}

impl BuffMeta for BuffGorouE1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouE1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "五郎-「大将旗指物」-1",
            en: "Gorou-「General's War Banner」-1",
        ),
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "五郎E技能：一名角色时：「坚牢」：防御力提升。",
            en: "五郎E技能：一名角色时：「坚牢」：防御力提升。",
        )),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill2",
            title: crate::common::i18n::locale!(
                zh_cn: "五郎E技能等级",
                en: "Gorou E Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let skill2 = match *b {
            BuffConfig::GorouE1 { skill2 } => skill2,
            _ => 1
        };

        Box::new(BuffGorouE1 {
            skill2
        })
    }
}

pub struct BuffGorouE3;

impl<A: Attribute> Buff<A> for BuffGorouE3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusGeo, "BUFF: 五郎-「大将旗指物」-3", 0.15);
    }
}

impl BuffMeta for BuffGorouE3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouE3,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "五郎-「大将旗指物」-3",
            en: "Gorou-「General's War Banner」-3",
        ),
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "五郎E技能：三名角色时：「摧碎」：除上述效果外，获得岩元素伤害加成。",
            en: "五郎E技能：三名角色时：「摧碎」：除上述效果外，获得岩元素伤害加成。",
        )),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGorouE3)
    }
}

pub struct BuffGorouTalent1;

impl<A: Attribute> Buff<A> for BuffGorouTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_def_percentage("BUFF: 五郎天赋「不畏风雨」", 0.25);
    }
}

impl BuffMeta for BuffGorouTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "五郎-「不畏风雨」",
            en: "Gorou-「Heedless of the Wind and Weather」",
        ),
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "五郎天赋1：施放兽牙逐突形胜战法后的12秒内，附近的队伍中所有角色的防御力提升25%。",
            en: "五郎天赋1：施放兽牙逐突形胜战法后的12秒内，附近的队伍中所有角色的防御力提升25%。",
        )),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGorouTalent1)
    }
}

pub struct BuffGorouC6 {
    pub level: usize
}

impl<A: Attribute> Buff<A> for BuffGorouC6 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = match self.level {
            1 => 0.1,
            2 => 0.2,
            _ => 0.4
        };

        attribute.set_value_by(AttributeName::CriticalDamageGeo, "BUFF：五郎六命「犬勇•忠如山」", value);
    }
}

impl BuffMeta for BuffGorouC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "五郎-「犬勇•忠如山」",
            en: "Gorou-「Valiant Hound: Mountainous Fealty」",
        ),
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "五郎命座6：施放犬坂吠吠方圆阵或兽牙逐突形胜战法后的12秒内，依据施放时的领域等级，提高附近的队伍中所有角色岩元素伤害的暴击伤害。",
            en: "五郎命座6：施放犬坂吠吠方圆阵或兽牙逐突形胜战法后的12秒内，依据施放时的领域等级，提高附近的队伍中所有角色岩元素伤害的暴击伤害。",
        )),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: crate::common::i18n::locale!(
                zh_cn: "领域等级",
                en: "Domain Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 3, default: 1 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let level = match *b {
            BuffConfig::GorouC6 { level } => level,
            _ => 1
        };
        Box::new(BuffGorouC6 {
            level
        })
    }
}
