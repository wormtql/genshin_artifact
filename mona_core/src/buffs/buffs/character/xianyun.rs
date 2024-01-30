use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffXianyunTalent1 {
    pub stack: f64,
}

impl<A: Attribute> Buff<A> for BuffXianyunTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = if self.stack < 1e-6 {
            0.0
        } else if self.stack >= 1e-6 && self.stack < 1.0 {
            self.stack * 0.04
        } else {
            0.02 + self.stack * 0.02
        };
        attribute.set_value_by(AttributeName::CriticalBase, "BUFF: 闲云天赋1「霜翎高逐祥风势」", bonus);
    }
}

impl BuffMeta for BuffXianyunTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianyunTalent1,
        name_locale: locale!(
            zh_cn: "闲云-「霜翎高逐祥风势」",
            en: "Xianun-「Galefeather Pursuit」"
        ),
        image: BuffImage::Avatar(CharacterName::Xianyun),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "朝起鹤云的闲云冲击波每命中一个敌人，都将为队伍中附近的所有角色产生一层持续20秒，至多叠加4层的「风翎」效果，使角色的下落攻击的暴击率提升4%/6%/8%/10%。每次命中敌人产生的「风翎」的持续时间独立计算。",
            en: "Each opponent hit by Driftcloud Waves from White Clouds at Dawn will grant all nearby party members 1 stack of Storm Pinion for 20s. Max 4 stacks. These will cause the characters' Plunging Attack CRIT Rate to increase by 4%/6%/8%/10% respectively. Each Storm Pinion created by hitting an opponent has an independent duration."
        )),
        from: BuffFrom::Character(CharacterName::Xianyun)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "层数",
                en: "Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::XianyunTalent1 { stack } => stack,
            _ => 0.0
        };
        Box::new(BuffXianyunTalent1 {
            stack
        })
    }
}

pub struct BuffXianyunTalent2 {
    pub rate: f64,
    pub c2: bool,
    pub atk: f64,
}

impl<A: Attribute> Buff<A> for BuffXianyunTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let factor = if self.c2 { 2.0 } else { 1.0 };
        let dmg = (9000.0_f64 * factor).min(self.atk * 2.0 * factor);
        attribute.set_value_by(AttributeName::ExtraDmgPlungingAttackLowHigh, "闲云天赋2「细想应是洞中仙」", dmg);
    }
}

impl BuffMeta for BuffXianyunTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianyunTalent2,
        name_locale: locale!(
            zh_cn: "闲云-「细想应是洞中仙」",
            en: "Xianyun-「Consider, the Adeptus in Her Realm」"
        ),
        image: BuffImage::Avatar(CharacterName::Xianyun),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "暮集竹星的竹星拥有仙力助推时，附近的当前场上角色的下落攻击坠地冲击造成的伤害提升，提升值相当于闲云的攻击力的200%。通过这种方式，至多使附近的当前场上角色的下落攻击坠地冲击伤害提升9000。<br>C2：固有天赋「细想应是洞中仙」的效果获得提升：暮集竹星的竹星拥有仙力助推时，附近的当前场上角色的下落攻击坠地冲击造成的伤害提升，提升值相当于闲云的攻击力的400%。通过这种方式，至多使附近的当前场上角色的下落攻击坠地冲击伤害提升18000。",
            en: "When the Starwicker created by Stars Gather at Dusk has Adeptal Assistance stacks, nearby active characters' Plunging Attack shockwave DMG will be increased by 200% of Xianyun's ATK. The maximum DMG increase that can be achieved this way is 9,000.<br>C2: the effects of the Passive Talent \"Consider, the Adeptus in Her Realm\" will be enhanced: When the Starwicker created by Stars Gather at Dusk has Adeptal Assistance stacks, nearby active characters' Plunging Attack shockwave DMG will be increased by 400% of Xianyun's ATK. The maximum DMG increase that can be achieved this way is 18,000."
        )),
        from: BuffFrom::Character(CharacterName::Xianyun)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "应用比例",
                en: "Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "c2",
            title: locale!(
                zh_cn: "命座2「鹤唳远人间」",
                en: "C2"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "atk",
            title: locale!(
                zh_cn: "闲云的攻击力",
                en: "ATK of Xianyun"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10000.0, default: 2000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate, c2, atk) = match *b {
            BuffConfig::XianyunTalent2 { rate, c2, atk } => (rate, c2, atk),
            _ => (0.0, false, 0.0)
        };
        Box::new(BuffXianyunTalent2 {
            rate, c2, atk
        })
    }
}
