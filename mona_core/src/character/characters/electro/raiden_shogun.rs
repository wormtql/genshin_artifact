use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::RaidenShogunDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct RaidenShogunSkill {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_q_bonus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_bonus1: [f64; 15],
    pub elemental_burst_bonus2: [f64; 15],
    pub elemental_burst_normal_dmg1: [f64; 15],
    pub elemental_burst_normal_dmg2: [f64; 15],
    pub elemental_burst_normal_dmg3: [f64; 15],
    pub elemental_burst_normal_dmg41: [f64; 15],
    pub elemental_burst_normal_dmg42: [f64; 15],
    pub elemental_burst_normal_dmg5: [f64; 15],
    pub elemental_burst_charged_dmg11: [f64; 15],
    pub elemental_burst_charged_dmg12: [f64; 15],
    pub elemental_burst_plunging_dmg1: [f64; 15],
    pub elemental_burst_plunging_dmg2: [f64; 15],
    pub elemental_burst_plunging_dmg3: [f64; 15],
}

pub const RAIDEN_SHOGUN_SKILL: RaidenShogunSkill = RaidenShogunSkill {
    normal_dmg1: [0.3965, 0.4287, 0.461, 0.5071, 0.5394, 0.5763, 0.627, 0.6777, 0.7284, 0.7837, 0.8471, 0.9216, 0.9962, 1.0707, 1.152],
    normal_dmg2: [0.3973, 0.4297, 0.462, 0.5082, 0.5405, 0.5775, 0.6283, 0.6791, 0.73, 0.7854, 0.8489, 0.9236, 0.9983, 1.073, 1.1545],
    normal_dmg3: [0.4988, 0.5394, 0.58, 0.638, 0.6786, 0.725, 0.7888, 0.8526, 0.9164, 0.986, 1.0658, 1.1595, 1.2533, 1.3471, 1.4494],
    normal_dmg41: [0.2898, 0.3134, 0.337, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192, 0.6737, 0.7282, 0.7827, 0.8422],
    normal_dmg42: [0.2898, 0.3134, 0.337, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192, 0.6737, 0.7282, 0.7827, 0.8422],
    normal_dmg5: [0.6545, 0.7077, 0.761, 0.8371, 0.8904, 0.9513, 1.035, 1.1187, 1.2024, 1.2937, 1.3983, 1.5214, 1.6444, 1.7675, 1.9017],
    charged_dmg1: [0.9959, 1.0769, 1.158, 1.2738, 1.3549, 1.4475, 1.5749, 1.7023, 1.8296, 1.9686, 2.1278, 2.3151, 2.5023, 2.6896, 2.8938],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.172, 1.2599, 1.3478, 1.465, 1.5529, 1.6408, 1.758, 1.8752, 1.9924, 2.1096, 2.2268, 2.344, 2.4905, 2.637, 2.7835],
    elemental_skill_dmg2: [0.42, 0.4515, 0.483, 0.525, 0.5565, 0.588, 0.63, 0.672, 0.714, 0.756, 0.798, 0.84, 0.8925, 0.945, 0.9975],
    elemental_skill_q_bonus: [0.0022, 0.0023, 0.0024, 0.0025, 0.0026, 0.0027, 0.0028, 0.0029, 0.003, 0.003, 0.003, 0.003, 0.003, 0.003, 0.003],
    elemental_burst_dmg1: [4.008, 4.3086, 4.6092, 5.01, 5.3106, 5.6112, 6.012, 6.4128, 6.8136, 7.2144, 7.6152, 8.016, 8.517, 9.018, 9.519],
    elemental_burst_bonus1: [0.0389, 0.0418, 0.0447, 0.0486, 0.0515, 0.0544, 0.0583, 0.0622, 0.0661, 0.07, 0.0739, 0.0778, 0.0826, 0.0875, 0.0923],
    elemental_burst_bonus2: [0.0073, 0.0078, 0.0084, 0.0091, 0.0096, 0.0102, 0.0109, 0.0116, 0.0123, 0.0131, 0.0138, 0.0145, 0.0154, 0.0163, 0.0172],
    elemental_burst_normal_dmg1: [0.4474, 0.4779, 0.5084, 0.5491, 0.5796, 0.6151, 0.6609, 0.7066, 0.7524, 0.7982, 0.8439, 0.8897, 0.9354, 0.9812, 1.0269],
    elemental_burst_normal_dmg2: [0.4396, 0.4695, 0.4995, 0.5395, 0.5694, 0.6044, 0.6494, 0.6943, 0.7393, 0.7842, 0.8292, 0.8741, 0.9191, 0.964, 1.009],
    elemental_burst_normal_dmg3: [0.5382, 0.5749, 0.6116, 0.6605, 0.6972, 0.74, 0.7951, 0.8501, 0.9052, 0.9602, 1.0153, 1.0703, 1.1254, 1.1804, 1.2355],
    elemental_burst_normal_dmg41: [0.3089, 0.3299, 0.351, 0.3791, 0.4001, 0.4247, 0.4563, 0.4879, 0.5195, 0.5511, 0.5827, 0.6143, 0.6458, 0.6774, 0.709],
    elemental_burst_normal_dmg42: [0.3098, 0.3309, 0.352, 0.3802, 0.4013, 0.4259, 0.4576, 0.4893, 0.521, 0.5526, 0.5843, 0.616, 0.6477, 0.6794, 0.711],
    elemental_burst_normal_dmg5: [0.7394, 0.7899, 0.8403, 0.9075, 0.9579, 1.0167, 1.0924, 1.168, 1.2436, 1.3192, 1.3948, 1.4705, 1.5461, 1.6217, 1.6973],
    elemental_burst_charged_dmg11: [0.616, 0.658, 0.7, 0.756, 0.798, 0.847, 0.91, 0.973, 1.036, 1.099, 1.162, 1.225, 1.288, 1.351, 1.414],
    elemental_burst_charged_dmg12: [0.7436, 0.7943, 0.845, 0.9126, 0.9633, 1.0225, 1.0985, 1.1746, 1.2506, 1.3267, 1.4027, 1.4788, 1.5548, 1.6309, 1.7069],
    elemental_burst_plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    elemental_burst_plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    elemental_burst_plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
};

pub const RAIDEN_SHOGUN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::RaidenShogun,
    internal_name: "Shougun",
    element: Element::Electro,
    hp: [1005, 2606, 3468, 5189, 5801, 6675, 7491, 8373, 8985, 9875, 10487, 11388, 12000, 12907],
    atk: [26, 68, 91, 136, 152, 174, 196, 219, 235, 258, 274, 298, 314, 337],
    def: [61, 159, 212, 317, 355, 408, 458, 512, 549, 604, 641, 696, 734, 789],
    sub_stat: CharacterSubStatFamily::Recharge320,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击•源流",
        en: "Normal Attack: Origin",
    ),
    skill_name2: locale!(
        zh_cn: "神变•恶曜开眼",
        en: "Transcendence: Baleful Omen",
    ),
    skill_name3: locale!(
        zh_cn: "奥义•梦想真说",
        en: "Secret Art: Musou Shinsetsu",
    ),
    name_locale: locale!(
        zh_cn: "雷电将军",
        en: "Raiden Shogun",
    )
};

pub struct RaidenShogunEffect {
    pub has_talent2: bool,
}

impl RaidenShogunEffect {
    pub fn new(common_data: &CharacterCommonData) -> RaidenShogunEffect {
        RaidenShogunEffect {
            has_talent2: common_data.has_talent2
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for RaidenShogunEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent2 {
            attribute.add_edge1(
                AttributeName::Recharge,
                AttributeName::BonusElectro,
                Box::new(|recharge, _| (recharge - 1.0) * 0.4),
                Box::new(|grad, _x1, _x2| (grad * 0.4, 0.0)),
                "雷电将军天赋：殊胜之御体"
            );
        }
    }
}

pub struct RaidenShogun;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum RaidenShogunDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal41,
    Normal42,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1,
    QNormal1,
    QNormal2,
    QNormal3,
    QNormal41,
    QNormal42,
    QNormal5,
    QCharged11,
    QCharged12,
    QPlunging1,
    QPlunging2,
    QPlunging3
}

impl RaidenShogunDamageEnum {
    pub fn get_element(&self) -> Element {
        use RaidenShogunDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal41 | Normal42 | Normal5 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            _ => Element::Electro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use RaidenShogunDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal41 | Normal42 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            _ => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for RaidenShogunDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum RaidenShogunRoleEnum {
    QTE
}

impl CharacterTrait for RaidenShogun {
    const STATIC_DATA: CharacterStaticData = RAIDEN_SHOGUN_STATIC_DATA;
    type SkillType = RaidenShogunSkill;
    const SKILL: Self::SkillType = RAIDEN_SHOGUN_SKILL;
    type DamageEnumType = RaidenShogunDamageEnum;
    type RoleEnum = RaidenShogunRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::E2 as usize, text: locale!(zh_cn: "协同攻击伤害", en: "Coordinated ATK DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::Q1 as usize, text: locale!(zh_cn: "梦想一刀基础伤害", en: "Musou no Hitotachi Base DMG") },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QNormal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QNormal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QNormal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QNormal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QNormal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QNormal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QCharged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QCharged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QPlunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QPlunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: RaidenShogunDamageEnum::QPlunging3 as usize, text: plunging_dmg!(3) },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "under_e",
            title: locale!(
                zh_cn: "处于雷罚恶曜之眼",
                en: "Under Eye of Stormy Judgment",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "resolve_stack",
            title: locale!(
                zh_cn: "诸愿百眼之愿力层数",
                en: "Chakra Desiderata Resolve",
            ),
            config: ItemConfigType::Int { min: 0, max: 60, default: 60 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: RaidenShogunDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use RaidenShogunDamageEnum::*;
        let ratio = match s {
            Normal1 => RAIDEN_SHOGUN_SKILL.normal_dmg1[s1],
            Normal2 => RAIDEN_SHOGUN_SKILL.normal_dmg2[s1],
            Normal3 => RAIDEN_SHOGUN_SKILL.normal_dmg3[s1],
            Normal41 => RAIDEN_SHOGUN_SKILL.normal_dmg41[s1],
            Normal42 => RAIDEN_SHOGUN_SKILL.normal_dmg42[s1],
            Normal5 => RAIDEN_SHOGUN_SKILL.normal_dmg5[s1],
            Charged => RAIDEN_SHOGUN_SKILL.charged_dmg1[s1],
            Plunging1 => RAIDEN_SHOGUN_SKILL.plunging_dmg1[s1],
            Plunging2 => RAIDEN_SHOGUN_SKILL.plunging_dmg2[s1],
            Plunging3 => RAIDEN_SHOGUN_SKILL.plunging_dmg3[s1],
            E1 => RAIDEN_SHOGUN_SKILL.elemental_skill_dmg1[s2],
            E2 => RAIDEN_SHOGUN_SKILL.elemental_skill_dmg2[s2],
            Q1 => RAIDEN_SHOGUN_SKILL.elemental_burst_dmg1[s3],
            QNormal1 => RAIDEN_SHOGUN_SKILL.elemental_burst_normal_dmg1[s3],
            QNormal2 => RAIDEN_SHOGUN_SKILL.elemental_burst_normal_dmg2[s3],
            QNormal3 => RAIDEN_SHOGUN_SKILL.elemental_burst_normal_dmg3[s3],
            QNormal41 => RAIDEN_SHOGUN_SKILL.elemental_burst_normal_dmg41[s3],
            QNormal42 => RAIDEN_SHOGUN_SKILL.elemental_burst_normal_dmg42[s3],
            QNormal5 => RAIDEN_SHOGUN_SKILL.elemental_burst_normal_dmg5[s3],
            QCharged11 => RAIDEN_SHOGUN_SKILL.elemental_burst_charged_dmg11[s3],
            QCharged12 => RAIDEN_SHOGUN_SKILL.elemental_burst_charged_dmg12[s3],
            QPlunging1 => RAIDEN_SHOGUN_SKILL.elemental_burst_plunging_dmg1[s3],
            QPlunging2 => RAIDEN_SHOGUN_SKILL.elemental_burst_plunging_dmg2[s3],
            QPlunging3 => RAIDEN_SHOGUN_SKILL.elemental_burst_plunging_dmg3[s3],
        };

        let (under_e, resolve_stack) = match *config {
            CharacterSkillConfig::RaidenShogun { under_e, resolve_stack } => (under_e, resolve_stack),
            _ => (false, 0)
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        let skill_type = s.get_skill_type();
        if skill_type == SkillType::ElementalBurst {
            if under_e {
                let bonus_per_energy = RAIDEN_SHOGUN_SKILL.elemental_skill_q_bonus[s2];
                builder.add_extra_bonus("雷罚恶曜之眼加成", bonus_per_energy * 90.0);
            }
            let resolve_bonus_per_stack = if s == RaidenShogunDamageEnum::Q1 {
                RAIDEN_SHOGUN_SKILL.elemental_burst_bonus1[s3]
            } else {
                RAIDEN_SHOGUN_SKILL.elemental_burst_bonus2[s3]
            };
            let extra_ratio = resolve_stack as f64 * resolve_bonus_per_stack;
            builder.add_atk_ratio("愿力加成", extra_ratio);

            let is_conste2 = context.character_common_data.constellation >= 2;
            if is_conste2 {
                builder.add_extra_def_penetration("雷电将军二命「斩铁断金」", 0.6);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(RaidenShogunEffect::new(common_data)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: RaidenShogunRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            RaidenShogunRoleEnum::QTE => Box::new(RaidenShogunDefaultTargetFunction {
                recharge_demand: 1.0
            })
        }
    }
}
