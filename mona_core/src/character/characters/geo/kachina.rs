use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::kachina::KachinaDamageEnum::{Normal1, Normal21, Normal22, Normal3, Normal4};
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

pub struct KachinaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg21: [f64; 15],
    pub normal_dmg22: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const KACHINA_SKILL: KachinaSkillType = KachinaSkillType {
    normal_dmg1: [0.494, 0.5342, 0.5744, 0.6318, 0.672, 0.718, 0.7812, 0.8444, 0.9076, 0.9765, 1.0454, 1.1143, 1.1833, 1.2522, 1.3211],
    normal_dmg21: [0.2757, 0.2981, 0.3206, 0.3526, 0.3751, 0.4007, 0.436, 0.4712, 0.5065, 0.545, 0.5834, 0.6219, 0.6604, 0.6988, 0.7373],
    normal_dmg22: [0.3063, 0.3313, 0.3562, 0.3918, 0.4167, 0.4452, 0.4844, 0.5236, 0.5628, 0.6055, 0.6483, 0.691, 0.7338, 0.7765, 0.8192],
    normal_dmg3: [0.7043, 0.7616, 0.8189, 0.9008, 0.9581, 1.0237, 1.1137, 1.2038, 1.2939, 1.3922, 1.4904, 1.5887, 1.687, 1.7852, 1.8835],
    normal_dmg4: [0.7744, 0.8374, 0.9004, 0.9905, 1.0535, 1.1255, 1.2246, 1.3236, 1.4227, 1.5307, 1.6388, 1.7468, 1.8549, 1.9629, 2.071],
    charged_dmg: [1.1266, 1.2183, 1.31, 1.441, 1.5327, 1.6375, 1.7816, 1.9257, 2.0698, 2.227, 2.3842, 2.5414, 2.6986, 2.8558, 3.013],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [0.8776, 0.9434, 1.0092, 1.097, 1.1628, 1.2286, 1.3164, 1.4042, 1.4919, 1.5797, 1.6674, 1.7552, 1.8649, 1.9746, 2.0843],
    e_dmg2: [0.6376, 0.6854, 0.7332, 0.797, 0.8448, 0.8926, 0.9564, 1.0202, 1.0839, 1.1477, 1.2114, 1.2752, 1.3549, 1.4346, 1.5143],
    q_dmg1: [3.8057, 4.1366, 4.4252, 4.81, 5.0986, 5.3872, 5.772, 6.1568, 6.5416, 6.9264, 7.3112, 7.696, 8.177, 8.658, 9.139],
};

damage_enum!(
    KachinaDamageEnum
    Normal1
    Normal21
    Normal22
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
);

impl KachinaDamageEnum {
    pub fn get_element(&self) -> Element {
        use KachinaDamageEnum::*;
        match *self {
            E1 | E2 | Q1 => Element::Geo,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KachinaDamageEnum::*;
        match *self {
            Normal1 | Normal21 | Normal22 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

pub struct KachinaEffect {
    pub talent1_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for KachinaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusGeo, "天赋「山的回声」", self.talent1_rate * 0.2);
    }
}

pub struct Kachina;

impl CharacterTrait for Kachina {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Kachina,
        internal_name: "Kachina",
        name_locale: locale!(
            zh_cn: "卡齐娜",
            en: "Kachina"
        ),
        element: Element::Geo,
        hp: [989, 2541, 3281, 4914, 5439, 6256, 6956, 7773, 8299, 9115, 9640, 10457, 10982, 11799],
        atk: [18, 47, 60, 90, 100, 115, 128, 143, 152, 167, 177, 192, 202, 217],
        def: [66, 171, 220, 330, 365, 420, 467, 522, 557, 612, 647, 702, 738, 792],
        sub_stat: CharacterSubStatFamily::Bonus240(StatName::GeoBonus),
        weapon_type: WeaponType::Polearm,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·嵴之啮咬",
            en: "Normal Attack: Cragbiter"
        ),
        skill_name2: locale!(
            zh_cn: "出击，冲天转转！",
            en: "Go, Go Turbo Twirly!"
        ),
        skill_name3: locale!(
            zh_cn: "现在，认真时间！",
            en: "Time to Get Serious!"
        ),
    };
    type SkillType = KachinaSkillType;
    const SKILL: Self::SkillType = KACHINA_SKILL;
    type DamageEnumType = KachinaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            KachinaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal21 hit_n_dmg!(2, 1)
            Normal22 hit_n_dmg!(2, 2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            KachinaDamageEnum
            E1 locale!(zh_cn: "冲天转转搭乘伤害", en: "Turbo Twirly Mounted DMG")
            E2 locale!(zh_cn: "冲天转转独立伤害", en: "Turbo Twirly Independent DMG")
        ),
        skill3: skill_map!(
            KachinaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「山的回声」比例",
                en: "Talent: Mountain Echoes Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KachinaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KachinaDamageEnum::*;

        let mut builder = D::new();
        let ratio = match s {
            Normal1 => KACHINA_SKILL.normal_dmg1[s1],
            Normal21 => KACHINA_SKILL.normal_dmg21[s1],
            Normal22 => KACHINA_SKILL.normal_dmg22[s1],
            Normal3 => KACHINA_SKILL.normal_dmg3[s1],
            Normal4 => KACHINA_SKILL.normal_dmg4[s1],
            Charged => KACHINA_SKILL.charged_dmg[s1],
            Plunging1 => KACHINA_SKILL.plunging_dmg1[s1],
            Plunging2 => KACHINA_SKILL.plunging_dmg2[s1],
            Plunging3 => KACHINA_SKILL.plunging_dmg3[s1],
            E1 => KACHINA_SKILL.e_dmg1[s2],
            E2 => KACHINA_SKILL.e_dmg2[s2],
            Q1 => KACHINA_SKILL.q_dmg1[s3],
        };

        let skill_type = s.get_skill_type();
        if skill_type == SkillType::ElementalSkill || skill_type == SkillType::ElementalBurst {
            builder.add_def_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        if context.character_common_data.has_talent2 {
            if skill_type == SkillType::ElementalSkill {
                builder.add_def_ratio("天赋「坚岩之重」", 0.2);
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
        match *config {
            CharacterConfig::Kachina { talent1_rate } => Some(Box::new(KachinaEffect {
                talent1_rate
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
