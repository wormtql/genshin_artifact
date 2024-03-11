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

pub struct ChioriSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_1_2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1_atk: [f64; 15],
    pub e_dmg1_def: [f64; 15],
    pub e_dmg2_atk: [f64; 15],
    pub e_dmg2_def: [f64; 15],

    pub q_dmg1_atk: [f64; 15],
    pub q_dmg1_def: [f64; 15],
}

pub const CHIORI_SKILL: ChioriSkillType = ChioriSkillType {
    normal_dmg1: [0.4941, 0.5343, 0.5745, 0.632, 0.6722, 0.7182, 0.7814, 0.8446, 0.9078, 0.9767, 1.0457, 1.1146, 1.1836, 1.2525, 1.3214],
    normal_dmg2: [0.4683, 0.5065, 0.5446, 0.599, 0.6372, 0.6807, 0.7406, 0.8005, 0.8604, 0.9258, 0.9911, 1.0565, 1.1218, 1.1872, 1.2525],
    normal_dmg3: [0.3042, 0.3289, 0.3537, 0.389, 0.4138, 0.4421, 0.481, 0.5199, 0.5588, 0.6013, 0.6437, 0.6861, 0.7286, 0.771, 0.8135],
    normal_dmg4: [0.7512, 0.8124, 0.8735, 0.9609, 1.022, 1.0919, 1.188, 1.2841, 1.3802, 1.485, 1.5898, 1.6946, 1.7995, 1.9043, 2.0091],
    charged_1_2: [0.5431, 0.5873, 0.6315, 0.6946, 0.7389, 0.7894, 0.8588, 0.9283, 0.9978, 1.0735, 1.1493, 1.2251, 1.3009, 1.3767, 1.4525],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1_atk: [0.8208, 0.8824, 0.9439, 1.026, 1.0876, 1.1491, 1.2312, 1.3133, 1.3954, 1.4774, 1.5595, 1.6416, 1.7442, 1.8468, 1.9494],
    e_dmg1_def: [1.026, 1.1029, 1.1799, 1.2825, 1.3594, 1.4364, 1.539, 1.6416, 1.7442, 1.8468, 1.9494, 2.052, 2.1802, 2.3085, 2.4367],
    e_dmg2_atk: [1.4928, 1.6048, 1.7167, 1.866, 1.978, 2.0899, 2.2392, 2.3885, 2.5378, 2.687, 2.8363, 2.9856, 3.1722, 3.3588, 3.5454],
    e_dmg2_def: [1.866, 2.0059, 2.1459, 2.3325, 2.4725, 2.6124, 2.799, 2.9856, 3.1722, 3.3588, 3.5454, 3.732, 3.9653, 4.1985, 4.4317],
    q_dmg1_atk: [2.5632, 2.7554, 2.9477, 3.204, 3.3962, 3.5885, 3.8448, 4.1011, 4.3574, 4.6138, 4.8701, 5.1264, 5.4468, 5.7672, 6.0876],
    q_dmg1_def: [3.204, 3.4443, 3.6846, 4.005, 4.2453, 4.4856, 4.806, 5.1264, 5.4468, 5.7672, 6.0876, 6.408, 6.8085, 7.209, 7.6095],
};

damage_enum!(
    ChioriDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal3
    Normal4
    Charged11
    Charged12
    Charged1
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3 // 「绢」
    Q1
);

impl ChioriDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use ChioriDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged11 | Charged12 | Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use ChioriDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 => Element::Geo,
            _ => Element::Physical
        }
    }
}

pub struct ChioriEffect {
    pub has_talent2: bool,
    pub talent2: bool,
}

impl<A: Attribute> ChangeAttribute<A> for ChioriEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent2 && self.talent2 {
            attribute.set_value_by(AttributeName::BonusGeo, "天赋「锦上添花」", 0.2);
        }
    }
}

pub struct Chiori;

impl CharacterTrait for Chiori {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Chiori,
        internal_name: "Chiori", // todo verify
        name_locale: locale!(
            zh_cn: "千织",
            en: "Chiori"
        ),
        element: Element::Geo,
        hp: [890, 2310, 3073, 4598, 5141, 5915, 6638, 7420, 7962, 8751, 9293, 10091, 10634, 11438],
        atk: [25, 65, 87, 130, 145, 167, 187, 209, 225, 247, 262, 285, 300, 323],
        def: [74, 192, 256, 383, 428, 493, 553, 618, 663, 729, 774, 841, 886, 953],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·心织刀流",
            en: "Normal Attack: Weaving Blade"
        ),
        skill_name2: locale!(
            zh_cn: "羽袖一触",
            en: "Fluttering Hasode"
        ),
        skill_name3: locale!(
            zh_cn: "二刀之形·比翼",
            en: "Hiyoku: Twin Blades"
        ),
    };
    type SkillType = ChioriSkillType;
    const SKILL: Self::SkillType = CHIORI_SKILL;
    type DamageEnumType = ChioriDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ChioriDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged11 charged_dmg!(1)
            Charged12 charged_dmg!(2)
            Charged1 charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ChioriDamageEnum
            E1 locale!(zh_cn: "袖伤害", en: "Tamoto DMG")
            E2 locale!(zh_cn: "上挑攻击伤害", en: "Upward Sweep Attack DMG")
            E3 locale!(zh_cn: "「绢」上海", en: "\"Kinu\" DMG")
        ),
        skill3: skill_map!(
            ChioriDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2",
            title: locale!(
                zh_cn: "天赋「锦上添花」",
                en: "Talent \"The Finishing Touch\""
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ChioriDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ChioriDamageEnum::*;
        let mut builder = D::new();
        let ratio_atk = match s {
            Normal1 => CHIORI_SKILL.normal_dmg1[s1],
            Normal2 => CHIORI_SKILL.normal_dmg2[s1],
            Normal31 | Normal32 => CHIORI_SKILL.normal_dmg3[s1],
            Normal3 => 2.0 * CHIORI_SKILL.normal_dmg3[s1],
            Normal4 => CHIORI_SKILL.normal_dmg4[s1],
            Charged11 | Charged12 => CHIORI_SKILL.charged_1_2[s1],
            Charged1 => 2.0 * CHIORI_SKILL.charged_1_2[s1],
            Plunging1 => CHIORI_SKILL.plunging_dmg1[s1],
            Plunging2 => CHIORI_SKILL.plunging_dmg2[s1],
            Plunging3 => CHIORI_SKILL.plunging_dmg3[s1],
            E1 => CHIORI_SKILL.e_dmg1_atk[s2],
            E2 => CHIORI_SKILL.e_dmg2_atk[s2],
            E3 => CHIORI_SKILL.e_dmg1_atk[s2] * 1.7,
            Q1 => CHIORI_SKILL.q_dmg1_atk[s3],
        };
        let ratio_def = match s {
            E1 => CHIORI_SKILL.e_dmg1_def[s2],
            E2 => CHIORI_SKILL.e_dmg2_def[s2],
            E3 => CHIORI_SKILL.e_dmg1_def[s2] * 1.7,
            Q1 => CHIORI_SKILL.q_dmg1_def[s3],
            _ => 0.0
        };
        builder.add_atk_ratio("技能倍率", ratio_atk);
        builder.add_def_ratio("技能倍率", ratio_def);

        let skill_type = s.get_skill_type();
        if skill_type == SkillType::NormalAttack && context.character_common_data.constellation >= 6 {
            // C6
            builder.add_def_ratio("C6「万理一空」", 2.35);
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
            CharacterConfig::Chiori { talent2 } => Some(Box::new(ChioriEffect {
                talent2,
                has_talent2: common_data.has_talent2
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
