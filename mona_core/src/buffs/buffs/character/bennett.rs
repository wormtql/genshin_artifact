use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Bennett;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffBennettQ {
    pub base_atk: f64,
    pub c1: bool,
    pub skill3: usize,
}

impl<A: Attribute> Buff<A> for BuffBennettQ {
    fn change_attribute(&self, attribute: &mut A) {
        let p = Bennett::SKILL.elemental_burst_atk_bonus[self.skill3 - 1] + (if self.c1 { 0.2 } else { 0.0 });
        let v = p * self.base_atk;

        attribute.set_value_by(AttributeName::ATKFixed, "BUFF：班尼特-「美妙旅程」", v);
    }
}

impl BuffMeta for BuffBennettQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BennettQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "班尼特-「美妙旅程」",
            en: "Bennett-「Fantastic Voyage」",
        ),
        image: BuffImage::Avatar(CharacterName::Bennett),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "班尼特Q技能：基于班尼特的基础攻击力，以一定比例获得攻击力加成<br>一命：美妙旅程的攻击力提升效果不再有血量限制，数值上追加班尼特基础攻击力的20%。",
            en: "Bennett Q: 基于班尼特的基础攻击力，以一定比例获得攻击力加成<br>一命：美妙旅程的攻击力提升效果不再有血量限制，数值上追加班尼特基础攻击力的20%。",
        )),
        from: BuffFrom::Character(CharacterName::Bennett),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "base_atk",
            title: crate::common::i18n::locale!(
                zh_cn: "班尼特的基础攻击力",
                en: "Bannett Base ATK",
            ),
            config: ItemConfigType::FloatInput { default: 800.0 },
        },
        ItemConfig {
            name: "c1",
            title: crate::common::i18n::locale!(
                zh_cn: "是否1命",
                en: "C1",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "skill3",
            title: crate::common::i18n::locale!(
                zh_cn: "技能等级",
                en: "Skill Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (base_atk, c1, skill3) = match *b {
            BuffConfig::BennettQ { base_atk, c1, skill3 } => (base_atk, c1, skill3),
            _ => (0.0, false, 1)
        };

        Box::new(BuffBennettQ {
            base_atk, c1, skill3
        })
    }
}

pub struct BuffBennettC6;

impl<A: Attribute> Buff<A> for BuffBennettC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "BUFF: 班尼特六命「烈火与勇气」", 0.15);
    }
}

impl BuffMeta for BuffBennettC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BennettC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "班尼特-「烈火与勇气」",
            en: "Bennett-「Fire Ventures With Me」",
        ),
        image: BuffImage::Avatar(CharacterName::Bennett),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "班尼特命座6：处在美妙旅程领域内的队伍中当前场上单手剑、双手剑、长柄武器角色获得15%火元素伤害加成<br>注：此处不管当前角色的武器类型",
            en: "Bennett C6: 处在美妙旅程领域内的队伍中当前场上单手剑、双手剑、长柄武器角色获得15%火元素伤害加成<br>注：此处不管当前角色的武器类型",
        )),
        from: BuffFrom::Character(CharacterName::Bennett)
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffBennettC6)
    }
}
