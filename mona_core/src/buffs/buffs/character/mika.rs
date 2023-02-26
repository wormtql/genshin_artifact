use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffMika {
    pub stack_talent2: f64,
    pub rate_c6: f64,
}

impl<A: Attribute> Buff<A> for BuffMika {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPhysical, "BUFF：米卡 -「灵风」", 0.1 * self.stack_talent2);
        attribute.set_value_by(AttributeName::CriticalDamagePhysical, "BUFF：米卡 -「灵风」", self.rate_c6 * 0.6);
    }
}

impl BuffMeta for BuffMika {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::Mika,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "米卡 -「灵风」",
            en: "Mika - Soulwind",
        ),
        image: BuffImage::Avatar(CharacterName::Mika),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "星霜的流旋的灵风状态将获得侦明效果强化，使角色处于场上时，造成的物理伤害提升10%；六命：处于灵风状态下的当前场上角色，其物理伤害的暴击伤害提高60%",
            en: "The Soulwind state caused by Starfrost Swirl will grant characters the Detector effect, increasing their Physical DMG by 10% when they are on the field; C6: Active characters affected by Soulwind will deal 60% more Physical CRIT DMG",
        )),
        from: BuffFrom::Character(CharacterName::Mika),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack_talent2",
            title: crate::common::i18n::locale!(
                zh_cn: "侦明等效层数",
                en: "Equivalent Stacks of Detector",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 3.0 },
        },
        ItemConfig {
            name: "rate_c6",
            title: crate::common::i18n::locale!(
                zh_cn: "六命比例",
                en: "C6 Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (stack_talent2, rate_c6) = match *b {
            BuffConfig::Mika { stack_talent2, rate_c6 } => (stack_talent2, rate_c6),
            _ => (0.0, 0.0)
        };

        Box::new(BuffMika {
            stack_talent2,
            rate_c6,
        })
    }
}
