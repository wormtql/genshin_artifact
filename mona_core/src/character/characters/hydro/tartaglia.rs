use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::TartagliaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct TartagliaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub normal_riptide_flash: [f64; 15],
    pub normal_riptide_burst: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_normal_dmg1: [f64; 15],
    pub elemental_skill_normal_dmg2: [f64; 15],
    pub elemental_skill_normal_dmg3: [f64; 15],
    pub elemental_skill_normal_dmg4: [f64; 15],
    pub elemental_skill_normal_dmg5: [f64; 15],
    pub elemental_skill_normal_dmg61: [f64; 15],
    pub elemental_skill_normal_dmg62: [f64; 15],
    pub elemental_skill_charged_dmg11: [f64; 15],
    pub elemental_skill_charged_dmg12: [f64; 15],
    pub elemental_skill_riptide_slash: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_riptide_blast: [f64; 15],
}

pub const TARTAGLIA_SKILL: TartagliaSkillType = TartagliaSkillType {
    normal_dmg1: [0.4128, 0.4464, 0.48, 0.528, 0.5616, 0.6, 0.6528, 0.7056, 0.7584, 0.816, 0.8736, 0.9312, 0.9888, 1.0464, 1.104],
    normal_dmg2: [0.4627, 0.5003, 0.538, 0.5918, 0.6295, 0.6725, 0.7317, 0.7909, 0.85, 0.9146, 0.9792, 1.0437, 1.1083, 1.1728, 1.2374],
    normal_dmg3: [0.5538, 0.5989, 0.644, 0.7084, 0.7535, 0.805, 0.8758, 0.9467, 1.0175, 1.0948, 1.1721, 1.2494, 1.3266, 1.4039, 1.4812],
    normal_dmg4: [0.5702, 0.6166, 0.663, 0.7293, 0.7757, 0.8288, 0.9017, 0.9746, 1.0475, 1.1271, 1.2067, 1.2862, 1.3658, 1.4453, 1.5249],
    normal_dmg5: [0.6089, 0.6584, 0.708, 0.7788, 0.8284, 0.885, 0.9629, 1.0408, 1.1186, 1.2036, 1.2886, 1.3735, 1.4585, 1.5434, 1.6284],
    normal_dmg6: [0.7276, 0.7868, 0.846, 0.9306, 0.9898, 1.0575, 1.1506, 1.2436, 1.3367, 1.4382, 1.5397, 1.6412, 1.7428, 1.8443, 1.9458],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    normal_riptide_flash: [0.124, 0.1333, 0.1426, 0.155, 0.1643, 0.1736, 0.186, 0.1984, 0.2108, 0.2232, 0.2356, 0.248, 0.2635, 0.279, 0.2945],
    normal_riptide_burst: [0.62, 0.6665, 0.713, 0.775, 0.8215, 0.868, 0.93, 0.992, 1.054, 1.116, 1.178, 1.24, 1.3175, 1.395, 1.4725],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.72, 0.774, 0.828, 0.9, 0.954, 1.008, 1.08, 1.152, 1.224, 1.296, 1.368, 1.44, 1.53, 1.62, 1.71],
    elemental_skill_normal_dmg1: [0.3887, 0.4204, 0.452, 0.4972, 0.5288, 0.565, 0.6147, 0.6644, 0.7142, 0.7684, 0.8226, 0.8769, 0.9311, 0.9854, 1.0396],
    elemental_skill_normal_dmg2: [0.4162, 0.4501, 0.484, 0.5324, 0.5663, 0.605, 0.6582, 0.7115, 0.7647, 0.8228, 0.8809, 0.939, 0.997, 1.0551, 1.1132],
    elemental_skill_normal_dmg3: [0.5633, 0.6092, 0.655, 0.7205, 0.7664, 0.8188, 0.8908, 0.9629, 1.0349, 1.1135, 1.1921, 1.2707, 1.3493, 1.4279, 1.5065],
    elemental_skill_normal_dmg4: [0.5994, 0.6482, 0.697, 0.7667, 0.8155, 0.8713, 0.9479, 1.0246, 1.1013, 1.1849, 1.2685, 1.3522, 1.4358, 1.5195, 1.6031],
    elemental_skill_normal_dmg5: [0.553, 0.598, 0.643, 0.7073, 0.7523, 0.8038, 0.8745, 0.9452, 1.0159, 1.0931, 1.1703, 1.2474, 1.3246, 1.4017, 1.4789],
    elemental_skill_normal_dmg61: [0.3543, 0.3832, 0.412, 0.4532, 0.482, 0.515, 0.5603, 0.6056, 0.651, 0.7004, 0.7498, 0.7993, 0.8487, 0.8982, 0.9476],
    elemental_skill_normal_dmg62: [0.3767, 0.4073, 0.438, 0.4818, 0.5125, 0.5475, 0.5957, 0.6439, 0.692, 0.7446, 0.7972, 0.8497, 0.9023, 0.9548, 1.0074],
    elemental_skill_charged_dmg11: [0.602, 0.651, 0.7, 0.77, 0.819, 0.875, 0.952, 1.029, 1.106, 1.19, 1.274, 1.358, 1.442, 1.526, 1.61],
    elemental_skill_charged_dmg12: [0.7198, 0.7784, 0.837, 0.9207, 0.9793, 1.0462, 1.1383, 1.2304, 1.3225, 1.4229, 1.5233, 1.6238, 1.7242, 1.8247, 1.9251],
    elemental_skill_riptide_slash: [0.602, 0.651, 0.7, 0.77, 0.819, 0.875, 0.952, 1.029, 1.106, 1.19, 1.274, 1.358, 1.442, 1.526, 1.61],
    elemental_burst_dmg1: [4.64, 4.988, 5.336, 5.8, 6.148, 6.496, 6.96, 7.424, 7.888, 8.352, 8.816, 9.28, 9.86, 10.44, 11.02],
    elemental_burst_dmg2: [3.784, 4.0678, 4.3516, 4.73, 5.0138, 5.2976, 5.676, 6.0544, 6.4328, 6.8112, 7.1896, 7.568, 8.041, 8.514, 8.987],
    elemental_burst_riptide_blast: [1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85]
};

pub const TARTAGLIA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Tartaglia,
    internal_name: "Tartaglia",
    element: Element::Hydro,
    hp: [1020, 2646, 3521, 5268, 5889, 6776, 7604, 8500, 9121, 10025, 10647, 11561, 12182, 13103],
    atk: [23, 61, 81, 121, 135, 156, 175, 195, 210, 231, 245, 266, 280, 301],
    def: [63, 165, 219, 328, 366, 421, 473, 528, 567, 623, 662, 719, 757, 815],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::HydroBonus),
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·断雨",
        en: "Normal Attack: Cutting Torrent",
    ),
    skill_name2: locale!(
        zh_cn: "魔王武装·狂澜",
        en: "Foul Legacy: Raging Tide",
    ),
    skill_name3: locale!(
        zh_cn: "极恶技·尽灭闪",
        en: "Havoc: Obliteration",
    ),
    name_locale: locale!(
        zh_cn: "达达利亚",
        en: "Tartaglia",
    )
};

pub struct Tartaglia;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum TartagliaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Normal6,
    Charged1,
    Charged2,
    NormalRiptideFlash,
    NormalRiptideBurst,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    ENormal1,
    ENormal2,
    ENormal3,
    ENormal4,
    ENormal5,
    ENormal61,
    ENormal62,
    ECharged11,
    ECharged12,
    ERiptideSlash,
    Q1,
    Q2,
    QRiptideBlast
}

impl TartagliaDamageEnum {
    pub fn get_element(&self) -> Element {
        use TartagliaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Normal6 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            _ => Element::Hydro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use TartagliaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Normal6 |
                ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 | ENormal61 | ENormal62 |
                NormalRiptideFlash | NormalRiptideBurst => SkillType::NormalAttack,
            Charged1 | Charged2 | ECharged11 | ECharged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | ERiptideSlash => SkillType::ElementalSkill,
            Q1 | Q2 | QRiptideBlast => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for TartagliaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum TartagliaRoleEnum {
    Main
}

impl CharacterTrait for Tartaglia {
    const STATIC_DATA: CharacterStaticData = TARTAGLIA_STATIC_DATA;
    type SkillType = TartagliaSkillType;
    const SKILL: Self::SkillType = TARTAGLIA_SKILL;
    type DamageEnumType = TartagliaDamageEnum;
    type RoleEnum = TartagliaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: TartagliaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Normal6 as usize, text: hit_n_dmg!(6) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::NormalRiptideFlash as usize, text: locale!(zh_cn: "断流·闪伤害", en: "Riptide Flash DMG") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::NormalRiptideBurst as usize, text: locale!(zh_cn: "断流·破伤害", en: "Riptide Burst DMG") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: TartagliaDamageEnum::E1 as usize, text: locale!(zh_cn: "状态爆发伤害", en: "Stance Change DMG") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal61 as usize, text: hit_n_dmg!(6, 1) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ENormal62 as usize, text: hit_n_dmg!(6, 2) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ECharged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ECharged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: TartagliaDamageEnum::ERiptideSlash as usize, text: locale!(zh_cn: "断流·斩伤害", en: "Riptide Slash") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: TartagliaDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害·近战", en: "Skill DMG: Melee") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::Q2 as usize, text: locale!(zh_cn: "技能伤害·远程", en: "Skill DMG: Ranged") },
            CharacterSkillMapItem { index: TartagliaDamageEnum::QRiptideBlast as usize, text: locale!(zh_cn: "断流·爆伤害", en: "Riptide Blast DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: TartagliaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use TartagliaDamageEnum::*;
        let ratio = match s {
            Normal1 => TARTAGLIA_SKILL.normal_dmg1[s1],
            Normal2 => TARTAGLIA_SKILL.normal_dmg2[s1],
            Normal3 => TARTAGLIA_SKILL.normal_dmg3[s1],
            Normal4 => TARTAGLIA_SKILL.normal_dmg4[s1],
            Normal5 => TARTAGLIA_SKILL.normal_dmg5[s1],
            Normal6 => TARTAGLIA_SKILL.normal_dmg6[s1],
            Charged1 => TARTAGLIA_SKILL.charged_dmg1[s1],
            Charged2 => TARTAGLIA_SKILL.charged_dmg2[s1],
            NormalRiptideFlash => TARTAGLIA_SKILL.normal_riptide_flash[s1],
            NormalRiptideBurst => TARTAGLIA_SKILL.normal_riptide_burst[s1],
            Plunging1 => TARTAGLIA_SKILL.plunging_dmg1[s1],
            Plunging2 => TARTAGLIA_SKILL.plunging_dmg2[s1],
            Plunging3 => TARTAGLIA_SKILL.plunging_dmg3[s1],
            E1 => TARTAGLIA_SKILL.elemental_skill_dmg1[s2],
            ENormal1 => TARTAGLIA_SKILL.elemental_skill_normal_dmg1[s2],
            ENormal2 => TARTAGLIA_SKILL.elemental_skill_normal_dmg2[s2],
            ENormal3 => TARTAGLIA_SKILL.elemental_skill_normal_dmg3[s2],
            ENormal4 => TARTAGLIA_SKILL.elemental_skill_normal_dmg4[s2],
            ENormal5 => TARTAGLIA_SKILL.elemental_skill_normal_dmg5[s2],
            ENormal61 => TARTAGLIA_SKILL.elemental_skill_normal_dmg61[s2],
            ENormal62 => TARTAGLIA_SKILL.elemental_skill_normal_dmg62[s2],
            ECharged11 => TARTAGLIA_SKILL.elemental_skill_charged_dmg11[s2],
            ECharged12 => TARTAGLIA_SKILL.elemental_skill_charged_dmg12[s2],
            ERiptideSlash => TARTAGLIA_SKILL.elemental_skill_riptide_slash[s2],
            Q1 => TARTAGLIA_SKILL.elemental_burst_dmg1[s3],
            Q2 => TARTAGLIA_SKILL.elemental_burst_dmg2[s3],
            QRiptideBlast => TARTAGLIA_SKILL.elemental_burst_riptide_blast[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: TartagliaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            TartagliaRoleEnum::Main => Box::new(TartagliaDefaultTargetFunction),
        }
    }
}
