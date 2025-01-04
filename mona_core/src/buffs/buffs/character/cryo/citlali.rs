use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffCitlaliTalent1 {
    pub has_c2: bool,
    pub rate: f64
}

impl<A: Attribute> Buff<A> for BuffCitlaliTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let down = 0.2 + if self.has_c2 { 0.2 } else { 0.0 };

        attribute.set_value_by(AttributeName::ResMinusPyro, "茜特菈莉「五重天的寒雨」", down * self.rate);
        attribute.set_value_by(AttributeName::ResMinusHydro, "茜特菈莉「五重天的寒雨」", down * self.rate);
    }
}

impl BuffMeta for BuffCitlaliTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CitlaliTalent1,
        name_locale: locale!(
            zh_cn: "茜特菈莉-「五重天的寒雨」",
            en: "Citlali-Mamaloaco's Frigid Rain"
        ),
        image: BuffImage::Avatar(CharacterName::Citlali),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "茜特菈莉天赋1。伊兹帕帕存在期间，队伍中附近的角色触发冻结反应或融化反应后，受本次反应影响的敌人的火元素与水元素抗性还会降低20%，持续12秒；此外，茜特菈莉还会恢复16点夜魂值，每8秒至多通过这种方式恢复一次夜魂值。\
                <br>C2：伊兹帕帕存在期间，队伍中附近的角色触发冻结反应或融化反应后，受本次反应影响的敌人的火元素与水元素抗性还会额外降低20%，持续12秒，该效果需要解锁固有天赋「五重天的寒雨」。",
            en: "Citlali Talent1. While Itzpapa is on the field, after nearby party members trigger the Frozen or Melt reactions, the Pyro and Hydro RES of opponents affected by that reaction decreases by 20% for 12s. Additionally, Citlali will regain 16 Nightsoul points. Nightsoul points can be restored this way once every 8s.\
                <br>C2: While Itzpapa is on the field, after nearby party members trigger Frozen or Melt, the opponent(s) affected by this reaction will have their Pyro and Hydro RES additionally decreased by 20% for 12s. You must first unlock the Passive Talent \"Mamaloaco's Frigid Rain\" to gain access to the above effect."
        )),
        from: BuffFrom::Character(CharacterName::Citlali),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "has_c2",
            title: locale!(
                zh_cn: "C2",
                en: "C2"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "比例",
                en: "Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (has_c2, rate) = match *b {
            BuffConfig::CitlaliTalent1 { has_c2, rate } => (has_c2, rate),
            _ => (false, 0.0)
        };
        Box::new(BuffCitlaliTalent1 {
            has_c2, rate
        })
    }
}

pub struct BuffCitlaliC1 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffCitlaliC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ExtraDmgBase, "茜特菈莉C1", 2.0 * self.em);
    }
}

impl BuffMeta for BuffCitlaliC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CitlaliC1,
        name_locale: locale!(
            zh_cn: "茜特菈莉-「四百星的芒刃」",
            en: "Citlali-Radiant Blades of Centzon Mimixcoah"
        ),
        image: BuffImage::Avatar(CharacterName::Citlali),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "施放元素战技霜昼黑星时，茜特菈莉将获得「白星之裙」效果，持续至伊兹帕帕退场：持续期间，茜特菈莉获得10层「星刃」，除茜特菈莉外的附近的当前场上角色的普通攻击、重击、下落攻击、元素战技或元素爆发造成伤害时，将消耗1层「星刃」，提升造成的伤害，提升值相当于茜特菈莉元素精通的200%。",
            en: "When she uses the Elemental Skill Dawnfrost Darkstar, Citlali gains the \"Opalstar Vestments\" effect until Itzpapa leaves the field. During this time, she will obtain 10 stacks of Stellar Blade. When a nearby active character other than Citlali deals DMG using Normal, Charged, or Plunging Attacks, or Elemental Skills and Bursts, 1 stack of Stellar Blade will be consumed to increase the DMG dealt by 200% of Citlali's Elemental Mastery."
        )),
        from: BuffFrom::Character(CharacterName::Citlali),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "茜特菈莉的元素精通",
                en: "EM of Citlali"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::CitlaliC1 { em } => em,
            _ => 0.0
        };
        Box::new(BuffCitlaliC1 {
            em
        })
    }
}

pub struct BuffCitlaliC6 {
    pub stack: f64
}

impl<A: Attribute> Buff<A> for BuffCitlaliC6 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = self.stack * 0.015;
        attribute.set_value_by(AttributeName::BonusPyro, "茜特菈莉C6", bonus);
        attribute.set_value_by(AttributeName::BonusHydro, "茜特菈莉C6", bonus);
    }
}

impl BuffMeta for BuffCitlaliC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CitlaliC6,
        name_locale: locale!(
            zh_cn: "茜特菈莉-「原动天的密契」",
            en: "Citlali-Teoiztac's Secret Pact"
        ),
        image: BuffImage::Avatar(CharacterName::Citlali),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "此外，施放元素战技霜昼黑星时，伊兹帕帕将消耗所有夜魂值，并在存在期间内持续消耗夜魂值；每通过上述方式消耗1点夜魂值，茜特菈莉就会获得1点「秘律之数」。<br>「秘律之数」上限为40点，基于茜特菈莉持有的「秘律之数」，每1点「秘律之数」会使附近的队伍中所有角色获得1.5%火元素与水元素伤害加成，并使茜特菈莉造成的伤害提升2.5%。",
            en: "Additionally, when Dawnfrost Darkstar is used, Itzpapa will consume all Nightsoul points, and while it is on the field, it will continuously consume Nightsoul points. Each Nightsoul point consumed in this way will grant Citlali 1 \"Cifra of the Secret Law\" point.<br>A maximum of 40 such points can be gained, and each point Citlali possesses will grant all nearby party members a 1.5% Pyro and Hydro DMG Bonus, and increase the DMG Citlali deals by 2.5%."
        )),
        from: BuffFrom::Character(CharacterName::Citlali),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "「秘律之数」",
                en: "Cifra of the Secret Law point"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 40.0, default: 30.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::CitlaliC6 { stack } => stack,
            _ => 0.0
        };
        Box::new(BuffCitlaliC6 {
            stack
        })
    }
}
