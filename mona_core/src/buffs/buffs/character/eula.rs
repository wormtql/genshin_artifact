use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Eula;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffEulaE {
    pub skill2: usize,
}

impl<A: Attribute> Buff<A> for BuffEulaE {
    fn change_attribute(&self, attribute: &mut A) {
        let p = Eula::SKILL.elemental_skill_res_cryo_minus[self.skill2 - 1];
        attribute.set_value_by(AttributeName::ResMinusCryo, "BUFF：优菈「冰潮的涡旋」", p);
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF：优菈「冰潮的涡旋」", p);
    }
}

impl BuffMeta for BuffEulaE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::EulaE,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "优菈-「冰潮的涡旋」减抗",
            en: "Eula-「Icetide Vortex」减抗",
        ),
        image: BuffImage::Avatar(CharacterName::Eula),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "优菈E技能：长按若消耗了冷酷之心效果，会使身边的敌人的物理抗性与冰元素抗性降低。",
            en: "Eula E: 长按若消耗了冷酷之心效果，会使身边的敌人的物理抗性与冰元素抗性降低。",
        )),
        from: BuffFrom::Character(CharacterName::Eula),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill2",
            title: crate::common::i18n::locale!(
                zh_cn: "技能等级",
                en: "Skill Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 9 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let skill2 = match *b {
            BuffConfig::EulaE { skill2 } => skill2,
            _ => 1
        };

        Box::new(BuffEulaE { skill2 })
    }
}
