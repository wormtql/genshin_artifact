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

pub struct ChascaSkillType {
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
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_dmg4: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
    pub q_dmg3: [f64; 15],
}

pub const CHASCA_SKILL: ChascaSkillType = ChascaSkillType {
    normal_dmg1: [0.4801, 0.5192, 0.5582, 0.6141, 0.6531, 0.6978, 0.7592, 0.8206, 0.882, 0.949, 1.016, 1.083, 1.15, 1.2169, 1.2839],
    normal_dmg2: [0.4459, 0.4822, 0.5185, 0.5703, 0.6066, 0.6481, 0.7051, 0.7622, 0.8192, 0.8814, 0.9436, 1.0058, 1.068, 1.1303, 1.1925],
    normal_dmg3: [0.297, 0.3211, 0.3453, 0.3799, 0.404, 0.4317, 0.4696, 0.5076, 0.5456, 0.587, 0.6285, 0.6699, 0.7114, 0.7528, 0.7942],
    normal_dmg4: [0.2547, 0.2754, 0.2961, 0.3257, 0.3465, 0.3702, 0.4027, 0.4353, 0.4679, 0.5034, 0.539, 0.5745, 0.61, 0.6456, 0.6811],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.6, 0.645, 0.69, 0.75, 0.795, 0.84, 0.9, 0.96, 1.02, 1.08, 1.14, 1.2, 1.275, 1.35, 1.425],
    e_dmg2: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
    e_dmg3: [0.488, 0.5246, 0.5612, 0.61, 0.6466, 0.6832, 0.732, 0.7808, 0.8296, 0.8784, 0.9272, 0.976, 1.037, 1.098, 1.159],
    e_dmg4: [1.6657, 1.7906, 1.9156, 2.0822, 2.2071, 2.332, 2.4986, 2.6652, 2.8317, 2.9983, 3.1649, 3.3314, 3.5397, 3.7479, 3.9561],
    q_dmg1: [0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98, 2.09],
    q_dmg2: [1.034, 1.1116, 1.1891, 1.2925, 1.3701, 1.4476, 1.551, 1.6544, 1.7578, 1.8612, 1.9646, 2.068, 2.1973, 2.3265, 2.4558],
    q_dmg3: [2.068, 2.2231, 2.3782, 2.585, 2.7401, 2.8952, 3.102, 3.3088, 3.5156, 3.7224, 3.9292, 4.136, 4.3945, 4.653, 4.9115],
};

damage_enum!(
    ChascaDamageEnum
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
    E2
    E3
    E4Pyro
    E4Hydro
    E4Cryo
    E4Electro
    Q1
    Q2
    Q3Pyro
    Q3Hydro
    Q3Cryo
    Q3Electro
    Talent2
    Talent2Pyro
    Talent2Hydro
    Talent2Cryo
    Talent2Electro
);

impl ChascaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use ChascaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | E2 => SkillType::NormalAttack,
            Charged1 | Charged2 | E3 | E4Pyro | E4Hydro | E4Cryo | E4Electro | Talent2
                | Talent2Pyro | Talent2Hydro | Talent2Cryo | Talent2Electro => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 | Q3Pyro | Q3Hydro | Q3Cryo | Q3Electro => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self) -> Element {
        use ChascaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            Charged2 | E1 | E2 | E3 | Q1 | Q2 | Talent2 => Element::Anemo,
            E4Pyro | Q3Pyro | Talent2Pyro => Element::Pyro,
            E4Hydro | Q3Hydro | Talent2Hydro => Element::Hydro,
            E4Cryo | Q3Cryo | Talent2Cryo => Element::Cryo,
            E4Electro | Q3Electro | Talent2Electro => Element::Electro
        }
    }
}

pub struct Chasca;

impl CharacterTrait for Chasca {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Chasca,
        internal_name: "Chasca",
        name_locale: locale!(
            zh_cn: "恰斯卡",
            en: "Chasca"
        ),
        element: Element::Anemo,
        hp: [763, 1978, 2632, 3939, 4403, 5066, 5686, 6355, 6820, 7495, 7960, 8643, 9108, 9797],
        atk: [27, 70, 93, 139, 156, 179, 201, 225, 241, 265, 282, 306, 322, 347],
        def: [48, 124, 165, 247, 276, 318, 357, 399, 428, 470, 500, 542, 572, 615],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Bow,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·迷羽流击",
            en: "Normal Attack: Phantom Feather Flurry"
        ),
        skill_name2: locale!(
            zh_cn: "灵缰追影",
            en: "Spirit Reins, Shadow Hunt"
        ),
        skill_name3: locale!(
            zh_cn: "索魂命袭",
            en: "Soul Reaper's Fatal Round"
        ),
    };
    type SkillType = ChascaSkillType;
    const SKILL: Self::SkillType = CHASCA_SKILL;
    type DamageEnumType = ChascaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ChascaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal3 hit_n_dmg!(3)
            Charged1 locale!(zh_cn: "瞄准射击", en: "Aimed Shot")
            Charged2 locale!(zh_cn: "满蓄力瞄准射击", en: "Fully-Charged Aimed Shot")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ChascaDamageEnum
            E1 locale!(zh_cn: "共鸣伤害", en: "Resonance DMG")
            E2 locale!(zh_cn: "多重瞄准点按伤害", en: "Multitarget Fire Tap DMG")
            E3 locale!(zh_cn: "追影弹伤害", en: "Shadowhunt Shell DMG")
            E4Pyro locale!(zh_cn: "焕光追影弹伤害-火", en: "Shining Shadowhunt Shell DMG-Pyro")
            E4Hydro locale!(zh_cn: "焕光追影弹伤害-水", en: "Shining Shadowhunt Shell DMG-Hydro")
            E4Cryo locale!(zh_cn: "焕光追影弹伤害-冰", en: "Shining Shadowhunt Shell DMG-Cryo")
            E4Electro locale!(zh_cn: "焕光追影弹伤害-雷", en: "Shining Shadowhunt Shell DMG-Electro")
            Talent2 locale!(zh_cn: "流焰追影弹", en: "Burning Shadowhunt Shot")
            Talent2Pyro locale!(zh_cn: "流焰追影弹-火", en: "Burning Shadowhunt Shot-Pyro")
            Talent2Hydro locale!(zh_cn: "流焰追影弹-水", en: "Burning Shadowhunt Shot-Hydro")
            Talent2Cryo locale!(zh_cn: "流焰追影弹-冰", en: "Burning Shadowhunt Shot-Cryo")
            Talent2Electro locale!(zh_cn: "流焰追影弹-雷", en: "Burning Shadowhunt Shot-Electro")
        ),
        skill3: skill_map!(
            ChascaDamageEnum
            Q1 locale!(zh_cn: "裂风索魂弹伤害", en: "Galesplitting Soulseeker Shell DMG")
            Q2 locale!(zh_cn: "索魂弹伤害", en: "Soulseeker Shell DMG")
            Q3Pyro locale!(zh_cn: "溢光索魂弹伤害-火", en: "Radiant Soulseeker Shell DMG-Pyro")
            Q3Hydro locale!(zh_cn: "溢光索魂弹伤害-水", en: "Radiant Soulseeker Shell DMG-Hydro")
            Q3Cryo locale!(zh_cn: "溢光索魂弹伤害-冰", en: "Radiant Soulseeker Shell DMG-Cryo")
            Q3Electro locale!(zh_cn: "溢光索魂弹伤害-雷", en: "Radiant Soulseeker Shell DMG-Electro")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element_count",
            title: locale!(zh_cn: "符合转化元素数量", en: "Element Count"),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 },
        },
        ItemConfig {
            name: "c6_rate",
            title: locale!(zh_cn: "C6效果比例", en: "C6 Ratio"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ChascaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ChascaDamageEnum::*;

        let mut builder = D::new();
        let ratio = match s {
            Normal1 => CHASCA_SKILL.normal_dmg1[s1],
            Normal2 => CHASCA_SKILL.normal_dmg2[s1],
            Normal3 => CHASCA_SKILL.normal_dmg3[s1],
            Normal4 => CHASCA_SKILL.normal_dmg4[s1],
            Charged1 => CHASCA_SKILL.charged_dmg1[s1],
            Charged2 => CHASCA_SKILL.charged_dmg2[s1],
            Plunging1 => CHASCA_SKILL.plunging_dmg1[s1],
            Plunging2 => CHASCA_SKILL.plunging_dmg2[s1],
            Plunging3 => CHASCA_SKILL.plunging_dmg3[s1],
            E1 => CHASCA_SKILL.e_dmg1[s2],
            E2 => CHASCA_SKILL.e_dmg2[s2],
            E3 => CHASCA_SKILL.e_dmg3[s2],
            E4Pyro | E4Hydro | E4Cryo | E4Electro => CHASCA_SKILL.e_dmg4[s2],
            Q1 => CHASCA_SKILL.q_dmg1[s3],
            Q2 => CHASCA_SKILL.q_dmg2[s3],
            Q3Pyro | Q3Hydro | Q3Cryo | Q3Electro => CHASCA_SKILL.q_dmg3[s3],
            Talent2 => CHASCA_SKILL.e_dmg3[s2] * 1.5,
            Talent2Pyro | Talent2Hydro | Talent2Cryo | Talent2Electro => CHASCA_SKILL.e_dmg4[s2] * 1.5,
        };
        builder.add_atk_ratio("技能倍率", ratio);

        let (element_count, c6_rate) = match *config {
            CharacterSkillConfig::Chasca { element_count, c6_rate } => (element_count, c6_rate),
            _ => (0, 0.0)
        };

        let is_e4 = s == E4Pyro || s == E4Hydro || s == E4Cryo || s == E4Electro;
        if context.character_common_data.has_talent1 && is_e4 {
            let bonus = if element_count == 1 {
                0.15
            } else if element_count == 2 {
                0.35
            } else if element_count == 3 {
                0.65
            } else {
                0.0
            };
            builder.add_extra_bonus("天赋「子弹的戏法」", bonus);
        }

        if context.character_common_data.has_talent1 && context.character_common_data.constellation >= 6
            && (is_e4 || s == E3) {
            let bonus = 1.2 * c6_rate;
            builder.add_extra_critical_damage("C6「相决，斗争的荣光」", bonus);
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
