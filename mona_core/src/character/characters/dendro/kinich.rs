use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::mualani::MualaniDamageEnum;
use crate::character::characters::navia::NaviaDamageEnum;
use crate::character::characters::navia::NaviaDamageEnum::{Plunging1, Plunging2, Plunging3};
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

pub struct KinichSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const KINICH_SKILL: KinichSkillType = KinichSkillType {
    normal_dmg1: [0.9899, 1.0704, 1.151, 1.2661, 1.3467, 1.4388, 1.5654, 1.692, 1.8186, 1.9567, 2.0948, 2.2329, 2.3711, 2.5092, 2.6473],
    normal_dmg2: [0.829, 0.8965, 0.964, 1.0604, 1.1279, 1.205, 1.311, 1.4171, 1.5231, 1.6388, 1.7545, 1.8702, 1.9858, 2.1015, 2.2172],
    normal_dmg3: [1.235, 1.3355, 1.436, 1.5796, 1.6801, 1.795, 1.953, 2.1109, 2.2689, 2.4412, 2.6135, 2.7858, 2.9582, 3.1305, 3.3028],
    normal_dmg4: [1.677, 1.8135, 1.95, 2.145, 2.2815, 2.4375, 2.652, 2.8665, 3.081, 3.315, 3.549, 3.783, 4.017, 4.251, 4.485],
    charged_dmg1: [0.4842, 0.5236, 0.563, 0.6193, 0.6587, 0.7038, 0.7657, 0.8276, 0.8895, 0.9571, 1.0247, 1.0922, 1.1598, 1.2273, 1.2949],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    e_dmg1: [0.5728, 0.6158, 0.6587, 0.716, 0.759, 0.8019, 0.8592, 0.9165, 0.9738, 1.031, 1.0883, 1.1456, 1.2172, 1.2888, 1.3604],
    e_dmg2: [6.8744, 7.39, 7.9056, 8.593, 9.1086, 9.6242, 10.3116, 10.999, 11.6865, 12.3739, 13.0614, 13.7488, 14.6081, 15.4674, 16.3267],
    q_dmg1: [1.34, 1.4405, 1.541, 1.675, 1.7755, 1.876, 2.01, 2.144, 2.278, 2.412, 2.546, 2.68, 2.8475, 3.015, 3.1825],
    q_dmg2: [1.2074, 1.2979, 1.3885, 1.5092, 1.5998, 1.6903, 1.811, 1.9318, 2.0525, 2.1732, 2.294, 2.4147, 2.5656, 2.7166, 2.8675],
};

damage_enum!(
    KinichDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E2_C2
    E2_C6
    Q1
    Q2
);

impl KinichDamageEnum {
    pub fn get_element(&self) -> Element {
        use KinichDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E1 | E2 | Q1 | Q2 | E2_C2 | E2_C6 => Element::Dendro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KinichDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E2_C2 | E2_C6 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

// pub struct KinichEffect {
//
// }

pub struct Kinich;

impl CharacterTrait for Kinich {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Kinich,
        internal_name: "Kinich",
        name_locale: locale!(
            zh_cn: "基尼奇",
            en: "Kinich"
        ),
        element: Element::Dendro,
        hp: [1001, 2597, 3455, 5170, 5779, 6649, 7462, 8341, 8951, 9838, 10448, 11345, 11954, 12858],
        atk: [26, 67, 89, 134, 149, 172, 193, 216, 231, 254, 270, 293, 309, 332],
        def: [62, 162, 215, 322, 360, 415, 465, 520, 558, 613, 651, 707, 745, 802],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Claymore,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·夜阳斗技",
            en: "Normal Attack: Nightsun Style"
        ),
        skill_name2: locale!(
            zh_cn: "悬猎·游骋高狩",
            en: "Canopy Hunter: Riding High"
        ),
        skill_name3: locale!(
            zh_cn: "向伟大圣龙致意",
            en: "Hail to the Almighty Dragonlord"
        ),
    };
    type SkillType = KinichSkillType;
    const SKILL: Self::SkillType = KINICH_SKILL;
    type DamageEnumType = KinichDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            KinichDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 locale!(zh_cn: "空中普通攻击伤害", en: "Mid-Air Normal Attack DMG")
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            KinichDamageEnum
            E1 locale!(zh_cn: "环绕射击伤害/2", en: "Loop Shot DMG/2")
            E2 locale!(zh_cn: "迴猎贯鳞炮伤害", en: "Scalespiker Cannon DMG")
            E2_C2 locale!(zh_cn: "迴猎贯鳞炮伤害（C2）", en: "Scalespiker Cannon DMG (C2)")
            E2_C6 locale!(zh_cn: "迴猎贯鳞炮伤害（C6）", en: "Scalespiker Cannon DMG (C6)")
        ),
        skill3: skill_map!(
            KinichDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "龙息伤害", en: "Dragon Breath DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hunters_exp",
            title: locale!(
                zh_cn: "「猎人心得」层数",
                en: "Hunter's Experience Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KinichDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();

        use KinichDamageEnum::*;

        let ratio = match s {
            Normal1 => KINICH_SKILL.normal_dmg1[s1],
            Normal2 => KINICH_SKILL.normal_dmg2[s1],
            Normal3 => KINICH_SKILL.normal_dmg3[s1],
            Normal4 => KINICH_SKILL.normal_dmg4[s1],
            Charged => KINICH_SKILL.charged_dmg1[s1],
            Plunging1 => KINICH_SKILL.plunging_dmg1[s1],
            Plunging2 => KINICH_SKILL.plunging_dmg2[s1],
            Plunging3 => KINICH_SKILL.plunging_dmg3[s1],
            E1 => KINICH_SKILL.e_dmg1[s2],
            E2 | E2_C2 => KINICH_SKILL.e_dmg1[s2],
            E2_C6 => 7.0,
            Q1 => KINICH_SKILL.q_dmg1[s3],
            Q2 => KINICH_SKILL.q_dmg2[s3],
        };

        builder.add_atk_ratio("技能倍率", ratio);

        // C2
        if (s == E2_C2 || s == E2_C6) && context.character_common_data.constellation >= 2 {
            builder.add_extra_bonus("C2「星虎之掌」", 1.0);
        }

        // Talent2
        let hunters_exp = match *config {
            CharacterSkillConfig::Kinich { hunters_exp } => hunters_exp,
            _ => 0.0
        };
        if hunters_exp > 0.0 && context.character_common_data.has_talent2 {
            if s == E2 || s == E2_C2 || s == E2_C6 {
                let bonus = 3.2 * hunters_exp;
                builder.add_atk_ratio("天赋「焰灵的契约」", bonus);
            }
        }

        // C1
        if (s == E2 || s == E2_C2 || s == E2_C6) && context.character_common_data.constellation >= 1 {
            builder.add_extra_critical_damage("C1「七鹦之喙」", 1.0);
        }

        // C4
        if (s == Q1 || s == Q2) && context.character_common_data.constellation >= 4 {
            builder.add_extra_bonus("C4「蜂鸟之羽」", 0.7);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
