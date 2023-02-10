use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::KamisatoAyato;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffKamisatoAyatoQ {
    pub skill_level: usize
}

impl<A: Attribute> Buff<A> for BuffKamisatoAyatoQ {
    fn change_attribute(&self, attribute: &mut A) {
        let value = KamisatoAyato::SKILL.elemental_burst_bonus[self.skill_level];
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: 神里绫人「水囿」", value);
    }
}

impl BuffMeta for BuffKamisatoAyatoQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KamisatoAyatoQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "神里绫人-「水囿」",
            en: "Ayato-「Suiyuu」",
        ),
        image: BuffImage::Avatar(CharacterName::KamisatoAyato),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "神里绫人Q技能：展开清净之园囿，熄灭其中一切嚣闹。存在期间，其中会持续降下水花剑，攻击范围内的敌人，造成水元素伤害，并提高其中的角色的普通攻击伤害。",
            en: "神里绫人Q技能：展开清净之园囿，熄灭其中一切嚣闹。存在期间，其中会持续降下水花剑，攻击范围内的敌人，造成水元素伤害，并提高其中的角色的普通攻击伤害。",
        )),
        from: BuffFrom::Character(CharacterName::KamisatoAyato)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill_level",
            title: crate::common::i18n::locale!(
                zh_cn: "神里绫人Q技能等级",
                en: "Ayato's Q Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let s = match *b {
            BuffConfig::KamisatoAyatoQ { skill_level } => skill_level,
            _ => 8
        };

        Box::new(BuffKamisatoAyatoQ {
            skill_level: (s - 1).clamp(0, 14)
        })
    }
}
