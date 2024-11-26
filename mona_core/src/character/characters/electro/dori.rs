use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait, CharacterSkillMapItem};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

skill_type!(
    DoriSkillType
    normal_dmg1
    normal_dmg21
    normal_dmg22
    normal_dmg3
    charged_dmg1
    charged_dmg2
    plunging_dmg1
    plunging_dmg2
    plunging_dmg3

    e1
    e2

    q1
    q_heal1
    q_heal1_fixed
);

pub const DORI_SKILL: DoriSkillType = DoriSkillType {
    normal_dmg1: [0.9021, 0.9756, 1.049, 1.1539, 1.2273, 1.3112, 1.4266, 1.542, 1.6574, 1.7833, 1.9092, 2.0351, 2.1609, 2.2868, 2.4127],
    normal_dmg21: [0.4107, 0.4442, 0.4776, 0.5254, 0.5588, 0.597, 0.6495, 0.7021, 0.7546, 0.8119, 0.8692, 0.9265, 0.9839, 1.0412, 1.0985],
    normal_dmg22: [0.4312, 0.4663, 0.5014, 0.5515, 0.5866, 0.6267, 0.6819, 0.7371, 0.7922, 0.8524, 0.9125, 0.9727, 1.0329, 1.0931, 1.1532],
    normal_dmg3: [1.284, 1.3885, 1.493, 1.6423, 1.7468, 1.8663, 2.0305, 2.1947, 2.3589, 2.5381, 2.7173, 2.8964, 3.0756, 3.2547, 3.4339],
    charged_dmg1: [0.6255, 0.6764, 0.7273, 0.8, 0.8509, 0.9091, 0.9891, 1.0691, 1.1491, 1.2364, 1.3364, 1.454, 1.5716, 1.6892, 1.8175],
    charged_dmg2: [1.1309, 1.2229, 1.315, 1.4465, 1.5386, 1.6437, 1.7884, 1.9331, 2.0777, 2.2355, 2.4163, 2.6289, 2.8416, 3.0542, 3.2862],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    e1: [1.4728, 1.5833, 1.6937, 1.841, 1.9515, 2.0619, 2.2092, 2.3565, 2.5038, 2.651, 2.7983, 2.9456, 3.1297, 3.3138, 3.4979],
    e2: [0.3156, 0.3393, 0.3629, 0.3945, 0.4182, 0.4418, 0.4734, 0.505, 0.5365, 0.5681, 0.5996, 0.6312, 0.6707, 0.7101, 0.7495],
    q1: [0.1588, 0.1707, 0.1826, 0.1985, 0.2104, 0.2224, 0.2382, 0.2541, 0.27, 0.2859, 0.3018, 0.3176, 0.3375, 0.3574, 0.3772],
    q_heal1: [0.0667, 0.0717, 0.0767, 0.0834, 0.0884, 0.0934, 0.1001, 0.1067, 0.1134, 0.1201, 0.1267, 0.1334, 0.1417, 0.1501, 0.1584],
    q_heal1_fixed: [641.98, 706.19, 775.74, 850.65, 930.91, 1016.52, 1107.48, 1203.79, 1305.45, 1412.46, 1524.82, 1642.54, 1765.6, 1894.01, 2027.78],
};

damage_enum!(
    DoriDamageEnum
    Normal1
    Normal21
    Normal22
    Normal3
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
    QHeal1
);

impl DoriDamageEnum {
    pub fn get_element(&self, flag: bool) -> Element {
        if flag {
            return Element::Electro
        }
        use DoriDamageEnum::*;
        match *self {
            E1 | E2 | Q1 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DoriDamageEnum::*;
        match *self {
            Normal1 | Normal21 | Normal22 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | QHeal1 => SkillType::ElementalBurst
        }
    }
}

pub struct Dori;

impl CharacterTrait for Dori {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Dori,
        internal_name: "Dori",
        element: Element::Electro,
        hp: [1039, 2670, 3447, 5163, 5715, 6573, 7309, 8168, 8719, 9577, 10129, 10987, 11539, 12397],
        atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 198, 208, 223],
        def: [61, 156, 201, 301, 333, 384, 427, 477, 509, 559, 591, 641, 673, 723],
        sub_stat: CharacterSubStatFamily::HP240,
        weapon_type: WeaponType::Claymore,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·妙显剑舞·改",
            en: "Normal Attack: Marvelous Sword-Dance (Modified)",
        ),
        skill_name2: locale!(
            zh_cn: "镇灵之灯·烦恼解决炮",
            en: "Spirit-Warding Lamp: Troubleshooter Cannon",
        ),
        skill_name3: locale!(
            zh_cn: "卡萨扎莱宫的无微不至",
            en: "Alcazarzaray's Exactitude",
        ),
        name_locale: locale!(
            zh_cn: "多莉",
            en: "Dori",
        )
    };
    type SkillType = DoriSkillType;
    const SKILL: Self::SkillType = DORI_SKILL;
    type DamageEnumType = DoriDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            DoriDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal21 hit_n_dmg!(2, 1)
            Normal22 hit_n_dmg!(2, 2)
            Normal3 hit_n_dmg!(3)
            Charged1 charged_dmg!("loop1")
            Charged2 charged_dmg!("loop2")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            DoriDamageEnum
            E1 locale!(zh_cn: "断除烦恼炮伤害", en: "Troubleshooter Shot DMG")
            E2 locale!(zh_cn: "售后服务弹伤害", en: "After-Sales Service Round DMG")
        ),
        skill3: skill_map!(
            DoriDamageEnum
            Q1 locale!(zh_cn: "连接伤害", en: "Connector DMG")
            QHeal1 locale!(zh_cn: "持续治疗量", en: "Continuous Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c6",
            title: locale!(
                zh_cn: "「漫掷万镒」",
                en: "C6「Sprinkling Weight」",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DoriDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use DoriDamageEnum::*;
        let ratio = damage_ratio!(
            DORI_SKILL
            s
            Normal1 normal_dmg1 s1
            Normal21 normal_dmg21 s1
            Normal22 normal_dmg22 s1
            Normal3 normal_dmg3 s1
            Charged1 charged_dmg1 s1
            Charged2 charged_dmg2 s1
            Plunging1 plunging_dmg1 s1
            Plunging2 plunging_dmg2 s1
            Plunging3 plunging_dmg3 s1
            E1 e1 s2
            E2 e2 s2
            Q1 q1 s3
            QHeal1 q_heal1 s3
        );

        if s == DoriDamageEnum::QHeal1 {
            let mut builder = D::new();
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", DORI_SKILL.q_heal1_fixed[s3]);
            builder.heal(&context.attribute)
        } else {
            let mut builder = D::new();
            builder.add_atk_ratio("技能倍率", ratio);

            let c6 = match *config {
                CharacterSkillConfig::Dori { c6 } => c6,
                _ => false
            };

            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(c6 && context.character_common_data.constellation >= 6),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}