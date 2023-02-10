use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::KujouSara;
use crate::character::prelude::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffKujouSaraEOrQ {
    pub c6: bool,
    pub base_atk: f64,
    pub skill2: usize,
}

impl<A: Attribute> Buff<A> for BuffKujouSaraEOrQ {
    fn change_attribute(&self, attribute: &mut A) {
        let p = KujouSara::SKILL.elemental_skill_atk_bonus[self.skill2 - 1];
        let atk_bonus = p * self.base_atk;
        attribute.set_value_by(AttributeName::ATKFixed, "BUFF: 九条裟罗「天狗咒雷」", atk_bonus);
        if self.c6 {
            attribute.set_value_by(AttributeName::CriticalDamageElectro, "BUFF: 九条裟罗六命「我界」", 0.6);
        }
    }
}

impl BuffMeta for BuffKujouSaraEOrQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KujouSaraEOrQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "九条裟罗-「天狗咒雷」",
            en: "KujouSara-「Tengu Juurai」",
        ),
        image: BuffImage::Avatar(CharacterName::KujouSara),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "九条裟罗E/Q技能：基于九条裟罗的基础攻击力，以一定比例获得攻击力加成<br>六命：处于天狗咒雷带来的攻击力提升效果状态下的角色，其雷元素伤害的暴击伤害提高60%。",
            en: "九条裟罗E/Q技能：基于九条裟罗的基础攻击力，以一定比例获得攻击力加成<br>六命：处于天狗咒雷带来的攻击力提升效果状态下的角色，其雷元素伤害的暴击伤害提高60%。",
        )),
        from: BuffFrom::Character(CharacterName::KujouSara),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "base_atk",
            title: crate::common::i18n::locale!(
                zh_cn: "九条裟罗的基础攻击力",
                en: "Sara Base ATK",
            ),
            config: ItemConfigType::FloatInput { default: 700.0 },
        },
        ItemConfig {
            name: "c6",
            title: crate::common::i18n::locale!(
                zh_cn: "是否6命",
                en: "C6",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "skill2",
            title: crate::common::i18n::locale!(
                zh_cn: "E技能等级",
                en: "E Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (base_atk, c6, skill2) = match *b {
            BuffConfig::KujouSaraEOrQ { base_atk, c6, skill2 } => (base_atk, c6, skill2),
            _ => (0.0, false, 1)
        };

        Box::new(BuffKujouSaraEOrQ {
            base_atk, c6, skill2
        })
    }
}
