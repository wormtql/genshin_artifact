use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffResonancePyro2;

impl<A: Attribute> Buff<A> for BuffResonancePyro2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: 元素共鸣-热诚之火", 0.25);
    }
}

impl BuffMeta for BuffResonancePyro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonancePyro2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-热诚之火",
            en: "Resonance-Fervent Flames",
        ),
        image: BuffImage::Misc("pyro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提升25%。",
            en: "攻击力提升25%。",
        )),
        from: BuffFrom::Resonance,
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffResonancePyro2)
    }
}


pub struct BuffResonanceCryo2 {
    pub rate: f64
}

impl<A: Attribute> Buff<A> for BuffResonanceCryo2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalAttacking, "元素共鸣-粉碎之冰", self.rate * 0.15);
    }
}

impl BuffMeta for BuffResonanceCryo2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceCryo2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-粉碎之冰",
            en: "Resonance-Shattering Ice",
        ),
        image: BuffImage::Misc("cryo"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "攻击冰元素附着或冻结状态下的敌人时，暴击率提高15%。",
            en: "攻击冰元素附着或冻结状态下的敌人时，暴击率提高15%。",
        )),
        from: BuffFrom::Resonance,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "应用比例",
                en: "Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let rate = match *b {
            BuffConfig::ResonanceCryo2 { rate } => rate,
            _ => 0.0
        };

        Box::new(BuffResonanceCryo2 {
            rate
        })
    }
}


pub struct BuffResonanceGeo2 {
    pub rate1: f64,
    pub rate2: f64
}

impl<A: Attribute> Buff<A> for BuffResonanceGeo2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ShieldStrength, "元素共鸣-坚定之岩", 0.15);
        attribute.set_value_by(AttributeName::BonusBase, "元素共鸣-坚定之岩", self.rate1 * 0.15);
        attribute.set_value_by(AttributeName::ResMinusGeo, "元素共鸣-坚定之岩", self.rate2 * 0.2);
    }
}

impl BuffMeta for BuffResonanceGeo2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceGeo2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-坚定之岩",
            en: "Resonance-Enduring Rock",
        ),
        image: BuffImage::Misc("geo"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "护盾强效提升15%。角色处于护盾保护状态时，①造成的伤害提升15%，对敌人造成伤害时会使敌人的的②岩元素抗性降低20%，持续15秒。",
            en: "护盾强效提升15%。角色处于护盾保护状态时，①造成的伤害提升15%，对敌人造成伤害时会使敌人的的②岩元素抗性降低20%，持续15秒。",
        )),
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: crate::common::i18n::locale!(
                zh_cn: "效果①比例",
                en: "Effect① Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: crate::common::i18n::locale!(
                zh_cn: "效果②比例",
                en: "Effect② Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate1, rate2) = match *b {
            BuffConfig::ResonanceGeo2 { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };

        Box::new(BuffResonanceGeo2 {
            rate1, rate2
        })
    }
}


pub struct BuffResonanceHydro2;

impl<A: Attribute> Buff<A> for BuffResonanceHydro2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_hp_percentage("元素共鸣-愈疗之水", 0.25);
    }
}

impl BuffMeta for BuffResonanceHydro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceHydro2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-愈疗之水",
            en: "Resonance-Soothing Water",
        ),
        image: BuffImage::Misc("hydro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "火元素附着的持续时间下降40%。生命值上限提升25%",
            en: "火元素附着的持续时间下降40%。生命值上限提升25%",
        )),
        from: BuffFrom::Resonance
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffResonanceHydro2)
    }
}

pub struct BuffResonanceDendro2 {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> Buff<A> for BuffResonanceDendro2 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = 50.0 + self.rate1 * 30.0 + self.rate2 * 20.0;
        attribute.set_value_by(AttributeName::ElementalMastery, "元素共鸣-蔓生之草", value);
    }
}

impl BuffMeta for BuffResonanceDendro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceDendro2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-蔓生之草",
            en: "Resonance-Sprawling Greenery",
        ),
        image: BuffImage::Misc("dendro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提升50点。触发燃烧、原激化、绽放反应后，队伍中附近的所有角色元素精通提升30点，持续6秒。触发超激化、蔓激化、超绽放、烈绽放反应后，队伍中附近的所有角色元素精通提升20点,持续6秒。以上效果的持续时间独立计算。",
            en: "元素精通提升50点。触发燃烧、原激化、绽放反应后，队伍中附近的所有角色元素精通提升30点，持续6秒。触发超激化、蔓激化、超绽放、烈绽放反应后，队伍中附近的所有角色元素精通提升20点,持续6秒。以上效果的持续时间独立计算。",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: crate::common::i18n::locale!(
                zh_cn: "效果①比例",
                en: "Effect① Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: crate::common::i18n::locale!(
                zh_cn: "效果②比例",
                en: "Effect② Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate1, rate2) = match *b {
            BuffConfig::ResonanceDendro2 { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };

        Box::new(BuffResonanceDendro2 {
            rate1, rate2
        })
    }
}
