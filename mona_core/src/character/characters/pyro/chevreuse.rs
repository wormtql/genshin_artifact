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

pub struct ChevreuseSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_heal: [f64; 15],
    pub e_heal_fixed: [f64; 15],
    pub e_dmg4: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const CHEVREUSE_SKILL: ChevreuseSkillType = ChevreuseSkillType {
    normal_dmg1: [0.5313, 0.5745, 0.6178, 0.6796, 0.7228, 0.7722, 0.8402, 0.9082, 0.9761, 1.0502, 1.1244, 1.1985, 1.2726, 1.3468, 1.4209],
    normal_dmg2: [0.4931, 0.5332, 0.5734, 0.6307, 0.6709, 0.7167, 0.7798, 0.8429, 0.9059, 0.9747, 1.0436, 1.1124, 1.1812, 1.25, 1.3188],
    normal_dmg31: [0.2764, 0.299, 0.3215, 0.3536, 0.3761, 0.4018, 0.4372, 0.4725, 0.5079, 0.5465, 0.585, 0.6236, 0.6622, 0.7008, 0.7393],
    normal_dmg32: [0.3245, 0.3509, 0.3774, 0.4151, 0.4415, 0.4717, 0.5132, 0.5547, 0.5962, 0.6415, 0.6868, 0.7321, 0.7774, 0.8226, 0.8679],
    normal_dmg4: [0.7726, 0.8355, 0.8984, 0.9882, 1.0511, 1.123, 1.2218, 1.3206, 1.4195, 1.5273, 1.6351, 1.7429, 1.8507, 1.9585, 2.0663],
    charged_dmg: [1.2169, 1.316, 1.415, 1.5565, 1.6556, 1.7687, 1.9244, 2.08, 2.2357, 2.4055, 2.5753, 2.7451, 2.9149, 3.0847, 3.2545],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [1.152, 1.2384, 1.3248, 1.44, 1.5264, 1.6128, 1.728, 1.8432, 1.9584, 2.0736, 2.1888, 2.304, 2.448, 2.592, 2.736],
    e_dmg2: [1.728, 1.8576, 1.9872, 2.16, 2.2896, 2.4192, 2.592, 2.7648, 2.9376, 3.1104, 3.2832, 3.456, 3.672, 3.888, 4.104],
    e_dmg3: [2.824, 3.0358, 3.2476, 3.53, 3.7418, 3.9536, 4.236, 4.5184, 4.8008, 5.0832, 5.3656, 5.648, 6.001, 6.354, 6.707],
    e_heal: [0.0267, 0.0287, 0.0307, 0.0333, 0.0353, 0.0373, 0.04, 0.0427, 0.0453, 0.048, 0.0507, 0.0533, 0.0567, 0.06, 0.0633],
    e_heal_fixed: [256.79, 282.47, 310.3, 340.26, 372.36, 406.61, 442.99, 481.52, 522.18, 564.98, 609.93, 657.01, 706.24, 757.61, 811.11],
    e_dmg4: [0.288, 0.3096, 0.3312, 0.36, 0.3816, 0.4032, 0.432, 0.4608, 0.4896, 0.5184, 0.5472, 0.576, 0.612, 0.648, 0.684],
    q_dmg1: [3.6816, 3.9577, 4.2338, 4.602, 4.8781, 5.1542, 5.5224, 5.8906, 6.2587, 6.6269, 6.995, 7.3632, 7.8234, 8.2836, 8.7438],
    q_dmg2: [0.4909, 0.5277, 0.5645, 0.6136, 0.6504, 0.6872, 0.7363, 0.7854, 0.8345, 0.8836, 0.9327, 0.9818, 1.0431, 1.1045, 1.1658],
};

damage_enum!(
    ChevreuseDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal4
    Charged
    Charged1 // same as Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    E4
    EHeal
    Q1
    Q2
);

impl ChevreuseDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use ChevreuseDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged | Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 | EHeal => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use ChevreuseDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4
            | Charged | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E1 | E2 | E3 | E4 | EHeal | Q1 | Q2 => Element::Pyro
        }
    }
}

pub struct ChevreuseEffect {
    pub talent1_rate: f64,
    pub talent2_rate: f64,
    pub constellation: usize,
    pub c6_stack: f64,
}

impl<A: Attribute> ChangeAttribute<A> for ChevreuseEffect {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPyro, "天赋「尖兵协同战法」", 0.4 * self.talent1_rate);
        attribute.set_value_by(AttributeName::ResMinusElectro, "天赋「尖兵协同战法」", 0.4 * self.talent1_rate);
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKPercentage,
            Box::new(|hp, _| -> f64 {
                let x = (hp / 1000.0).floor();
                (x * 0.01).min(0.4)
            }),
            Box::new(|_x, _y, _grad| (0.0, 0.0)),
            "天赋「纵阵武力统筹」"
        );
        if self.constellation >= 6 {
            attribute.set_value_by(AttributeName::BonusPyro, "C6「终结罪恶的追缉」", 0.2 * self.c6_stack);
            attribute.set_value_by(AttributeName::BonusElectro, "C6「终结罪恶的追缉」", 0.2 * self.c6_stack);
        }
    }
}

pub struct Chevreuse;

impl CharacterTrait for Chevreuse {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Chevreuse,
        internal_name: "Chevreuse",
        name_locale: locale!(
            zh_cn: "夏沃蕾",
            en: "Chevreuse"
        ),
        element: Element::Pyro,
        hp: [1003, 2577, 3326, 4982, 5514, 6343, 7052, 7881, 8413, 9241, 9773, 10602, 11134, 11962],
        atk: [16, 42, 54, 80, 89, 102, 114, 127, 136, 149, 158, 171, 180, 193],
        def: [51, 130, 168, 252, 279, 321, 357, 398, 425, 467, 494, 536, 563, 605],
        sub_stat: CharacterSubStatFamily::HP240,
        weapon_type: WeaponType::Polearm,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·线列枪刺·改",
            en: "Normal Attack: Line Bayonet Thrust EX"
        ),
        skill_name2: locale!(
            zh_cn: "近迫式急促拦射",
            en: "Short-Range Rapid Interdiction Fire"
        ),
        skill_name3: locale!(
            zh_cn: "圆阵掷弹爆轰术",
            en: "Ring of Bursting Grenades"
        )
    };
    type SkillType = ChevreuseSkillType;
    const SKILL: Self::SkillType = CHEVREUSE_SKILL;
    type DamageEnumType = ChevreuseDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ChevreuseDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ChevreuseDamageEnum
            E1 locale!(zh_cn: "点按伤害", en: "Press DMG")
            E2 locale!(zh_cn: "长按伤害", en: "Hold DMG")
            E3 locale!(zh_cn: "「超量装药弹头」伤害", en: "Overcharged Ball DMG")
            E4 locale!(zh_cn: "流涌之刃伤害", en: "Surging Blade DMG")
            EHeal locale!(zh_cn: "持续治疗量", en: "HP Regeneration Over Time")
        ),
        skill3: skill_map!(
            ChevreuseDamageEnum
            Q1 locale!(zh_cn: "爆轰榴弹伤害", en: "Explosive Grenade DMG")
            Q2 locale!(zh_cn: "二重毁伤弹伤害", en: "Secondary Explosive Shell DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋1「尖兵协同战法」比例",
                en: "Talent 「Vanguard's Coordinated Tactics」 Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「纵阵武力统筹」比例",
                en: "Talent 「Vertical Force Coordination」 Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "c6_stack",
            title: locale!(
                zh_cn: "C6层数",
                en: "C6 Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ChevreuseDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ChevreuseDamageEnum::*;
        let mut builder = D::new();
        if s == ChevreuseDamageEnum::EHeal {
            let ratio = CHEVREUSE_SKILL.e_heal[s2];
            let fixed = CHEVREUSE_SKILL.e_heal_fixed[s2];
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => CHEVREUSE_SKILL.normal_dmg1[s1],
                Normal2 => CHEVREUSE_SKILL.normal_dmg2[s1],
                Normal31 => CHEVREUSE_SKILL.normal_dmg31[s1],
                Normal32 => CHEVREUSE_SKILL.normal_dmg32[s1],
                Normal4 => CHEVREUSE_SKILL.normal_dmg4[s1],
                Charged | Charged1 => CHEVREUSE_SKILL.charged_dmg[s1],
                Plunging1 => CHEVREUSE_SKILL.plunging_dmg1[s1],
                Plunging2 => CHEVREUSE_SKILL.plunging_dmg2[s1],
                Plunging3 => CHEVREUSE_SKILL.plunging_dmg3[s1],
                E1 => CHEVREUSE_SKILL.e_dmg1[s2],
                E2 => CHEVREUSE_SKILL.e_dmg2[s2],
                E3 => CHEVREUSE_SKILL.e_dmg3[s2],
                E4 => CHEVREUSE_SKILL.e_dmg4[s2],
                Q1 => CHEVREUSE_SKILL.q_dmg1[s3],
                Q2 => CHEVREUSE_SKILL.q_dmg2[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Chevreuse { talent1_rate, talent2_rate, c6_stack } => Some(Box::new(ChevreuseEffect {
                talent1_rate,
                talent2_rate,
                constellation: common_data.constellation as usize,
                c6_stack
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
