use crate::{attribute::Attribute, character::macros::damage_enum, common::ChangeAttribute, attribute::AttributeCommon};
use crate::attribute::AttributeName;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait};
use crate::common::{Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, plunging_dmg};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::traits::{CharacterSkillMapItem};

pub struct KiraraSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub charged_dmg13: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_shield1: [f64; 15],
    pub e_shield1_fixed: [f64; 15],
    pub e_shield2: [f64; 15],
    pub e_shield2_fixed: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const KIRARA_SKILL: KiraraSkillType = KiraraSkillType {
    normal_dmg1: [0.479, 0.518, 0.557, 0.6127, 0.6517, 0.6963, 0.7575, 0.8188, 0.8801, 0.9469, 1.0137, 1.0806, 1.1474, 1.2143, 1.2811],
    normal_dmg2: [0.4635, 0.5013, 0.539, 0.5929, 0.6306, 0.6737, 0.733, 0.7923, 0.8516, 0.9163, 0.981, 1.0457, 1.1103, 1.175, 1.2397],
    normal_dmg31: [0.2542, 0.2749, 0.2956, 0.3252, 0.3459, 0.3695, 0.402, 0.4345, 0.467, 0.5025, 0.538, 0.5735, 0.6089, 0.6444, 0.6799],
    normal_dmg32: [0.3813, 0.4124, 0.4434, 0.4877, 0.5188, 0.5543, 0.603, 0.6518, 0.7006, 0.7538, 0.807, 0.8602, 0.9134, 0.9666, 1.0198],
    normal_dmg4: [0.7327, 0.7924, 0.852, 0.9372, 0.9968, 1.065, 1.1587, 1.2524, 1.3462, 1.4484, 1.5506, 1.6529, 1.7551, 1.8574, 1.9596],
    charged_dmg11: [0.2238, 0.242, 0.2602, 0.2862, 0.3044, 0.3252, 0.3539, 0.3825, 0.4111, 0.4423, 0.4736, 0.5048, 0.536, 0.5672, 0.5985],
    charged_dmg12: [0.4475, 0.484, 0.5204, 0.5724, 0.6089, 0.6505, 0.7077, 0.765, 0.8222, 0.8847, 0.9471, 1.0096, 1.072, 1.1345, 1.1969],
    charged_dmg13: [0.4475, 0.484, 0.5204, 0.5724, 0.6089, 0.6505, 0.7077, 0.765, 0.8222, 0.8847, 0.9471, 1.0096, 1.072, 1.1345, 1.1969],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [1.04, 1.118, 1.196, 1.3, 1.378, 1.456, 1.56, 1.664, 1.768, 1.872, 1.976, 2.08, 2.21, 2.34, 2.47],
    e_shield1: [0.1, 0.1075, 0.115, 0.125, 0.1325, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.2, 0.2125, 0.225, 0.2375],
    e_shield1_fixed: [962.23, 1058.47, 1162.72, 1275.0, 1395.3, 1523.61, 1659.95, 1804.3, 1956.68, 2117.07, 2285.48, 2461.92, 2646.37, 2838.84, 3039.34],
    e_shield2: [0.16, 0.172, 0.184, 0.2, 0.212, 0.224, 0.24, 0.256, 0.272, 0.288, 0.304, 0.32, 0.34, 0.36, 0.38],
    e_shield2_fixed: [1541.08, 1695.21, 1862.18, 2042.0, 2234.66, 2440.17, 2658.52, 2889.71, 3133.75, 3390.63, 3660.36, 3942.93, 4238.34, 4546.6, 4867.71],
    e_dmg2: [0.336, 0.3612, 0.3864, 0.42, 0.4452, 0.4704, 0.504, 0.5376, 0.5712, 0.6048, 0.6384, 0.672, 0.714, 0.756, 0.798],
    e_dmg3: [1.44, 1.548, 1.656, 1.8, 1.908, 2.016, 2.16, 2.304, 2.448, 2.592, 2.736, 2.88, 3.06, 3.24, 3.42],
    q_dmg1: [5.7024, 6.1301, 6.5578, 7.128, 7.5557, 7.9834, 8.5536, 9.1238, 9.6941, 10.2643, 10.8346, 11.4048, 12.1176, 12.8304, 13.5432],
    q_dmg2: [0.3564, 0.3831, 0.4099, 0.4455, 0.4722, 0.499, 0.5346, 0.5702, 0.6059, 0.6415, 0.6772, 0.7128, 0.7574, 0.8019, 0.8464],
};

damage_enum!(
    KiraraDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal4
    Charged11
    Charged12
    Charged13
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    Q1
    Q2
);

impl KiraraDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use KiraraDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged11 | Charged12 | Charged13 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use KiraraDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 | Q2 => Element::Dendro,
            _ => Element::Physical
        }
    }
}

pub struct Kirara;

impl CharacterTrait for Kirara {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Kirara,
        internal_name: "Momoka",
        name_locale: locale!(zh_cn: "绮良良", en: "Kirara"),
        element: Element::Dendro,
        hp: [1021, 2623, 3386, 5072, 5614, 6485, 7181, 8024, 8566, 9409, 9951, 10794, 11336, 12180],
        atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 198, 208, 223],
        def: [46, 118, 152, 227, 252, 290, 322, 360, 384, 422, 446, 484, 508, 546],
        sub_stat: CharacterSubStatFamily::HP240,
        weapon_type: WeaponType::Sword,
        star: 4,
        skill_name1: locale!(zh_cn: "普通攻击·箱纸切削术", en: "Normal Attack: Boxcutter"),
        skill_name2: locale!(zh_cn: "呜喵町飞足", en: "Meow-teor Kick"),
        skill_name3: locale!(zh_cn: "秘法·惊喜特派", en: "Secret Art: Surprise Dispatch")
    };
    type SkillType = KiraraSkillType;
    const SKILL: Self::SkillType = KIRARA_SKILL;
    type DamageEnumType = KiraraDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            KiraraDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Charged11 locale!(zh_cn: "重击伤害-1", en: "Charnged DMG-1")
            Charged12 locale!(zh_cn: "重击伤害-2", en: "Charnged DMG-2")
            Charged13 locale!(zh_cn: "重击伤害-3", en: "Charnged DMG-3")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            KiraraDamageEnum
            E1 locale!(zh_cn: "甩尾飞踢伤害", en: "Tail-Flicking Flying Kick DMG")
            E2 locale!(zh_cn: "猫箱急件冲撞伤害", en: "Urgent Neko Parcel Hit DMG")
            E3 locale!(zh_cn: "翻正爪击伤害", en: "Flipclaw Strike DMG")
        ),
        skill3: skill_map!(
            KiraraDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "猫草豆蔻爆炸伤害", en: "Cat Grass Cardamom Explosion DMG")
        )
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KiraraDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();
        use KiraraDamageEnum::*;

        let ratio = match s {
            Normal1 => KIRARA_SKILL.normal_dmg1[s1],
            Normal2 => KIRARA_SKILL.normal_dmg2[s1],
            Normal31 => KIRARA_SKILL.normal_dmg31[s1],
            Normal32 => KIRARA_SKILL.normal_dmg32[s1],
            Normal4 => KIRARA_SKILL.normal_dmg4[s1],
            Charged11 => KIRARA_SKILL.charged_dmg11[s1],
            Charged12 => KIRARA_SKILL.charged_dmg12[s1],
            Charged13 => KIRARA_SKILL.charged_dmg13[s1],
            Plunging1 => KIRARA_SKILL.plunging_dmg1[s1],
            Plunging2 => KIRARA_SKILL.plunging_dmg2[s1],
            Plunging3 => KIRARA_SKILL.plunging_dmg3[s1],
            E1 => KIRARA_SKILL.e_dmg1[s2],
            E2 => KIRARA_SKILL.e_dmg2[s2],
            E3 => KIRARA_SKILL.e_dmg3[s2],
            Q1 => KIRARA_SKILL.q_dmg1[s3],
            Q2 => KIRARA_SKILL.q_dmg2[s3],
        };

        builder.add_atk_ratio("技能倍率", ratio);

        let skill_type = s.get_skill_type();

        if context.character_common_data.has_talent2 {
            let hp = context.attribute.get_hp();
            let level = (hp / 1000.0).floor();
            if skill_type == SkillType::ElementalSkill {
                builder.add_extra_bonus("天赋「应时惑目之灵」", level * 0.004);
            } else if skill_type == SkillType::ElementalBurst {
                builder.add_extra_bonus("天赋「应时惑目之灵」", level * 0.003);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
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