use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct GamingSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_heal: f64,
}

pub const GAMING_SKILL: GamingSkillType = GamingSkillType {
    normal_dmg1: [0.8386, 0.9068, 0.9751, 1.0726, 1.1408, 1.2188, 1.3261, 1.4334, 1.5406, 1.6576, 1.7746, 1.8916, 2.0086, 2.1257, 2.2427],
    normal_dmg2: [0.7904, 0.8548, 0.9191, 1.011, 1.0754, 1.1489, 1.25, 1.3511, 1.4522, 1.5625, 1.6728, 1.7831, 1.8934, 2.0037, 2.114],
    normal_dmg3: [1.0665, 1.1533, 1.2401, 1.3641, 1.4509, 1.5501, 1.6865, 1.8229, 1.9593, 2.1081, 2.2569, 2.4057, 2.5545, 2.7034, 2.8522],
    normal_dmg4: [1.2795, 1.3836, 1.4878, 1.6366, 1.7407, 1.8597, 2.0234, 2.187, 2.3507, 2.5292, 2.7078, 2.8863, 3.0648, 3.2434, 3.4219],
    charged_dmg1: [0.6252, 0.6761, 0.727, 0.7997, 0.8506, 0.9087, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231, 1.4104, 1.4976, 1.5849, 1.6721],
    charged_dmg2: [1.1309, 1.2229, 1.315, 1.4465, 1.5386, 1.6437, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.6415, 0.6937, 0.7459, 0.8205, 0.8727, 0.9323, 1.0144, 1.0964, 1.1785, 1.268, 1.3575, 1.447, 1.5365, 1.626, 1.7155],
    plunging_dmg2: [1.2826, 1.387, 1.4914, 1.6406, 1.745, 1.8643, 2.0284, 2.1924, 2.3565, 2.5354, 2.7144, 2.8934, 3.0724, 3.2513, 3.4303],
    plunging_dmg3: [1.6021, 1.7325, 1.8629, 2.0492, 2.1796, 2.3286, 2.5335, 2.7384, 2.9434, 3.1669, 3.3905, 3.614, 3.8376, 4.0611, 4.2846],
    e_dmg1: [2.304, 2.4768, 2.6496, 2.88, 3.0528, 3.2256, 3.456, 3.6864, 3.9168, 4.1472, 4.3776, 4.608, 4.896, 5.184, 5.472],
    q_dmg1: [3.704, 3.9818, 4.2596, 4.63, 4.9078, 5.1856, 5.556, 5.9264, 6.2968, 6.6672, 7.0376, 7.408, 7.871, 8.334, 8.797],
    q_heal: 0.3
};

pub struct GamingEffect {
    pub hp_above50: bool,
    pub has_talent2: bool,
    pub c2_rate: f64,
    pub constellation: usize,
}

impl<A: Attribute> ChangeAttribute<A> for GamingEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.constellation >= 2 {
            attribute.add_atk_percentage("C2「步踏梅花」", 0.2 * self.c2_rate);
        }
        if self.has_talent2 {
            if self.hp_above50 {
                attribute.set_value_by(AttributeName::USER1, "talent2_bonus", 0.2);
            } else {
                attribute.set_value_by(AttributeName::IncomingHealingBonus, "天赋「祥烟瑞气」", 0.2);
            }
        }
    }
}

damage_enum!(
    GamingDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
    QHeal
);

impl GamingDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use GamingDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 | E1 => SkillType::PlungingAttackOnGround,
            Q1 | QHeal => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self, pyro: bool) -> Element {
        use GamingDamageEnum::*;
        if pyro {
            return Element::Pyro;
        }
        match *self {
            E1 | Q1 => Element::Pyro,
            _ => Element::Physical
        }
    }
}

pub struct Gaming;

impl CharacterTrait for Gaming {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Gaming,
        internal_name: "Gaming",
        name_locale: locale!(
            zh_cn: "嘉明",
            en: "Gaming"
        ),
        element: Element::Pyro,
        hp: [957, 2460, 3175, 4755, 5264, 6054, 6732, 7523, 8031, 8821, 9329, 10120, 10628, 11419],
        atk: [25, 65, 84, 126, 139, 160, 178, 199, 212, 233, 246, 267, 281, 302],
        def: [59, 151, 195, 293, 324, 373, 414, 463, 494, 543, 574, 623, 654, 703],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Claymore,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·刃爪悬星",
            en: "Normal Attack: Stellar Rend"
        ),
        skill_name2: locale!(
            zh_cn: "瑞兽登高楼",
            en: "Bestial Ascent"
        ),
        skill_name3: locale!(
            zh_cn: "璨焰金猊舞",
            en: "Suanni's Gilded Dance"
        )
    };
    type SkillType = GamingSkillType;
    const SKILL: Self::SkillType = GAMING_SKILL;
    type DamageEnumType = GamingDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            GamingDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged1 charged_dmg!("loop1")
            Charged2 charged_dmg!("loop2")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            GamingDamageEnum
            E1 locale!(zh_cn: "下落攻击·踏云献瑞伤害", en: "Plunging Attack: Charmed Cloudstrider DMG")
        ),
        skill3: skill_map!(
            GamingDamageEnum
            Q1 locale!(zh_cn: "猊兽·文仔砸击伤害", en: "Suanni Man Chai Smash DMG")
            QHeal locale!(zh_cn: "技能治疗量", en: "Skill Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_above50",
            title: locale!(
                zh_cn: "生命值大于等于50%",
                en: "HP ≥ 50%"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "c2_rate",
            title: locale!(
                zh_cn: "C2「步踏梅花」",
                en: "C2 'Plum Blossoms Underfoot'"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "pyro",
            title: locale!(
                zh_cn: "舞兽态势",
                en: "Wushou Stance"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: GamingDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use GamingDamageEnum::*;
        let mut builder = D::new();

        if s == QHeal {
            builder.add_hp_ratio("技能倍率", GAMING_SKILL.q_heal);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => GAMING_SKILL.normal_dmg1[s1],
                Normal2 => GAMING_SKILL.normal_dmg2[s1],
                Normal3 => GAMING_SKILL.normal_dmg3[s1],
                Normal4 => GAMING_SKILL.normal_dmg4[s1],
                Charged1 => GAMING_SKILL.charged_dmg1[s1],
                Charged2 => GAMING_SKILL.charged_dmg2[s1],
                Plunging1 => GAMING_SKILL.plunging_dmg1[s1],
                Plunging2 => GAMING_SKILL.plunging_dmg2[s1],
                Plunging3 => GAMING_SKILL.plunging_dmg3[s1],
                E1 => GAMING_SKILL.e_dmg1[s2],
                Q1 => GAMING_SKILL.q_dmg1[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);
            let pyro = match *config {
                CharacterSkillConfig::Gaming { pyro } => pyro,
                _ => false
            };
            let element = s.get_element(pyro);
            let skill_type = s.get_skill_type();

            builder.add_extra_bonus("天赋「祥烟瑞气」", context.attribute.get_value(AttributeName::USER1));
            if context.character_common_data.constellation >= 6 {
                builder.add_extra_critical("C6「百兽俱驯」", 0.2);
                builder.add_extra_critical_damage("C6「百兽俱驯」", 0.4);
            }

            builder.damage(
                &context.attribute,
                &context.enemy,
                element,
                skill_type,
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Gaming { hp_above50, c2_rate } => {
                Some(Box::new(GamingEffect {
                    hp_above50,
                    c2_rate,
                    has_talent2: common_data.has_talent2,
                    constellation: common_data.constellation as usize,
                }))
            },
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
