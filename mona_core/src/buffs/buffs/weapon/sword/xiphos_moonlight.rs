use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffXiphosMoonlight {
    pub em: f64,
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffXiphosMoonlight {
    fn change_attribute(&self, attribute: &mut A) {
        let per = self.refine as f64 * 0.00009 + 0.00027;
        let value = per * self.em * 0.3;

        attribute.set_value_by(AttributeName::Recharge, "BUFF: 西福斯的月光被动", value);
    }
}

impl BuffMeta for BuffXiphosMoonlight {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XiphosMoonlight,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "西福斯的月光-「镇灵的低语」",
            en: "Xiphos’ Moonlight-「Whisper of the Jinn」",
        ),
        image: BuffImage::Weapon(WeaponName::XiphosMoonlight),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "装备者的每点元素精通，都会为该角色提升0.036%/0.045%/0.054%/0.063%/0.072%元素充能效率，并基于该提升的30%为队伍中附近的其他角色提升元素充能效率，持续12秒，多件同名武器产生的此效果可以叠加。角色处于队伍后台时也能触发效果。",
            en: "The equipping character will gain 0.036%/0.045%/0.054%/0.063%/0.072% Energy Recharge for each point of Elemental Mastery they possess for 12s, with nearby party members gaining 30% of this buff for the same duration. Multiple instances of this weapon can allow this buff to stack. This effect will still trigger even if the character is not on the field.",
        )),
        from: BuffFrom::Weapon(WeaponName::XiphosMoonlight)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "元素精通",
                en: "EM"
            ),
            config: ItemConfigType::FloatInput { default: 900.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (em, refine) = match *b {
            BuffConfig::XiphosMoonlight { em, refine } => (em, refine),
            _ => (0.0, 1)
        };

        Box::new(BuffXiphosMoonlight {
            em, refine
        })
    }
}
