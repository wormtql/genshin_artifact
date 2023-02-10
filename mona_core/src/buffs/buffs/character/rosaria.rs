use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffRosariaTalent2 {
    pub crit: f64,
}

impl<A: Attribute> Buff<A> for BuffRosariaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let c = (self.crit * 0.15).clamp(0.0, 0.15);
        attribute.set_value_by(AttributeName::CriticalBase, "BUFF: 罗莎莉亚天赋「暗中支援的黯色」", c);
    }
}

impl BuffMeta for BuffRosariaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RosariaTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "罗莎莉亚-「暗中支援的黯色」",
            en: "Rosaria-「Shadow Samaritan」",
        ),
        image: BuffImage::Avatar(CharacterName::Rosaria),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "罗莎莉亚天赋2：施放终命的圣礼时，基于自身暴击率的15%，提高附近的队伍中所有角色(不包括罗莎莉亚自己)的暴击率，持续10秒。通过这种方式获得的暴击率提升，无法超过15%。",
            en: "罗莎莉亚天赋2：施放终命的圣礼时，基于自身暴击率的15%，提高附近的队伍中所有角色(不包括罗莎莉亚自己)的暴击率，持续10秒。通过这种方式获得的暴击率提升，无法超过15%。",
        )),
        from: BuffFrom::Character(CharacterName::Rosaria),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "crit",
            title: crate::common::i18n::locale!(
                zh_cn: "罗莎莉亚的暴击率",
                en: "Rosaria's Crit Rate",
            ),
            config: ItemConfigType::FloatPercentageInput { default: 70.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let v = match *b {
            BuffConfig::RosariaTalent2 { crit } => crit / 100.0,
            _ => 0.0
        };

        Box::new(BuffRosariaTalent2 {
            crit: v
        })
    }
}


pub struct BuffRosariaC6;

impl<A: Attribute> Buff<A> for BuffRosariaC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: 罗莎莉亚六命「代行裁判」", 0.2);
    }
}

impl BuffMeta for BuffRosariaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RosariaC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "罗莎莉亚-「代行裁判」",
            en: "Rosaria-「Divine Retribution」",
        ),
        image: BuffImage::Avatar(CharacterName::Rosaria),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "罗莎莉亚命座6：终命的圣礼的攻击会使敌人的物理抗性降低20%，持续10秒。",
            en: "罗莎莉亚命座6：终命的圣礼的攻击会使敌人的物理抗性降低20%，持续10秒。",
        )),
        from: BuffFrom::Character(CharacterName::Rosaria),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffRosariaC6)
    }
}
