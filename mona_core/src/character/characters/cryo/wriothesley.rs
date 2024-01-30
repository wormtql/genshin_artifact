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

pub struct WriothesleySkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_bonus: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const WRIOTHESLEY_SKILL: WriothesleySkillType = WriothesleySkillType {
    normal_dmg1: [0.5336, 0.577, 0.6205, 0.6825, 0.7259, 0.7756, 0.8438, 0.9121, 0.9803, 1.0548, 1.1292, 1.2037, 1.2781, 1.3526, 1.4271],
    normal_dmg2: [0.518, 0.5601, 0.6023, 0.6625, 0.7047, 0.7529, 0.8191, 0.8854, 0.9517, 1.0239, 1.0962, 1.1685, 1.2408, 1.313, 1.3853],
    normal_dmg3: [0.6722, 0.7269, 0.7817, 0.8598, 0.9145, 0.9771, 1.0631, 1.149, 1.235, 1.3288, 1.4226, 1.5164, 1.6102, 1.704, 1.7978],
    normal_dmg4: [0.379, 0.4099, 0.4407, 0.4848, 0.5157, 0.5509, 0.5994, 0.6479, 0.6964, 0.7493, 0.8022, 0.855, 0.9079, 0.9608, 1.0137],
    normal_dmg5: [0.9074, 0.9813, 1.0551, 1.1607, 1.2345, 1.3189, 1.435, 1.5511, 1.6671, 1.7937, 1.9204, 2.047, 2.1736, 2.3002, 2.4268],
    charged_dmg: [1.5296, 1.6443, 1.759, 1.912, 2.0267, 2.1414, 2.2944, 2.4474, 2.6003, 2.7533, 2.9062, 3.0592, 3.2504, 3.4416, 3.6328],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_bonus: [1.4317, 1.4575, 1.4834, 1.517, 1.5429, 1.5687, 1.6023, 1.6359, 1.6695, 1.7031, 1.7367, 1.7703, 1.8039, 1.8375, 1.8711],
    q_dmg1: [1.272, 1.3674, 1.4628, 1.59, 1.6854, 1.7808, 1.908, 2.0352, 2.1624, 2.2896, 2.4168, 2.544, 2.703, 2.862, 3.021],
    q_dmg2: [0.424, 0.4558, 0.4876, 0.53, 0.5618, 0.5936, 0.636, 0.6784, 0.7208, 0.7632, 0.8056, 0.848, 0.901, 0.954, 1.007],
};

damage_enum!(
    WriothesleyDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4Div2
    Normal5
    Charged
    ChargedTalent1
    Plunging1
    Plunging2
    Plunging3
    Q1
    Q2
);

impl WriothesleyDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use WriothesleyDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4Div2 | Normal5 => SkillType::NormalAttack,
            Charged | ChargedTalent1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

pub struct WriothesleyEffect {
    pub has_talent2: bool,
    pub talent2_stack: f64,
}

impl<A: Attribute> ChangeAttribute<A> for WriothesleyEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let value = 0.06 * self.talent2_stack;
        if self.has_talent2 {
            attribute.add_atk_percentage("天赋「罪业终有报偿之时」", value);
            attribute.set_value_to(AttributeName::USER1, "", self.talent2_stack);
        }
    }
}

pub struct Wriothesley;

impl CharacterTrait for Wriothesley {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Wriothesley,
        internal_name: "Wriothesley",
        name_locale: locale!(
            zh_cn: "莱欧斯利",
            en: "Wriothesley"
        ),
        element: Element::Cryo,
        hp: [1058, 2745, 3652, 5465, 6110, 7029, 7889, 8818, 9462, 10400, 11045, 11993, 12637, 13593],
        atk: [24, 63, 84, 125, 140, 161, 180, 202, 216, 238, 253, 274, 289, 311],
        def: [59, 154, 205, 307, 343, 395, 443, 495, 531, 584, 620, 673, 710, 763],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·迅烈倾霜拳",
            en: "Normal Attack: Forceful Fists of Frost"
        ),
        skill_name2: locale!(
            zh_cn: "冰牙突驰",
            en: "Icefang Rush"
        ),
        skill_name3: locale!(
            zh_cn: "黑金狼噬",
            en: "Darkgold Wolfbite"
        )
    };
    type SkillType = WriothesleySkillType;
    const SKILL: Self::SkillType = WRIOTHESLEY_SKILL;
    type DamageEnumType = WriothesleyDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            WriothesleyDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4Div2 locale!(
                zh_cn: "四段伤害/2",
                en: "4-Hit DMG/2"
            )
            Normal5 hit_n_dmg!(5)
            Charged charged_dmg!()
            ChargedTalent1 locale!(zh_cn: "惩戒·凌跃拳", en: "Rebuke: Vaulting Fist")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: None,
        skill3: skill_map!(
            WriothesleyDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "流涌之刃伤害", en: "Surging Blade DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_stack",
            title: locale!(
                zh_cn: "「检偿之敕」层数",
                en: "「Prosecution Edict」Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 4.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "under_chilling_penalty",
            title: locale!(
                zh_cn: "「寒烈的惩裁」状态",
                en: "Enable 「Chilling Penalty」",
            ),
            config: ItemConfigType::Bool { default: true },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: WriothesleyDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use WriothesleyDamageEnum::*;

        let under_chilling_penalty = match *config {
            CharacterSkillConfig::Wriothesley { under_chilling_penalty } 
              => under_chilling_penalty,
            _ => true,
        };

        let mut ratio = match s {
            Normal1 => WRIOTHESLEY_SKILL.normal_dmg1[s1] * (if under_chilling_penalty {WRIOTHESLEY_SKILL.e_bonus[s2]} else { 1.0 }),
            Normal2 => WRIOTHESLEY_SKILL.normal_dmg2[s1] * (if under_chilling_penalty {WRIOTHESLEY_SKILL.e_bonus[s2]} else { 1.0 }),
            Normal3 => WRIOTHESLEY_SKILL.normal_dmg3[s1] * (if under_chilling_penalty {WRIOTHESLEY_SKILL.e_bonus[s2]} else { 1.0 }),
            Normal4Div2 => WRIOTHESLEY_SKILL.normal_dmg4[s1] * (if under_chilling_penalty {WRIOTHESLEY_SKILL.e_bonus[s2]} else { 1.0 }),
            Normal5 => WRIOTHESLEY_SKILL.normal_dmg5[s1] * (if under_chilling_penalty {WRIOTHESLEY_SKILL.e_bonus[s2]} else { 1.0 }),
            Charged => WRIOTHESLEY_SKILL.charged_dmg[s1],
            ChargedTalent1 => WRIOTHESLEY_SKILL.charged_dmg[s1],
            Plunging1 => WRIOTHESLEY_SKILL.plunging_dmg1[s1],
            Plunging2 => WRIOTHESLEY_SKILL.plunging_dmg2[s1],
            Plunging3 => WRIOTHESLEY_SKILL.plunging_dmg3[s1],
            Q1 => WRIOTHESLEY_SKILL.q_dmg1[s3],
            Q2 => WRIOTHESLEY_SKILL.q_dmg2[s3],
        };

        let mut builder = D::new();

        let skill_type = s.get_skill_type();
        if skill_type == SkillType::ElementalBurst {
            if context.character_common_data.constellation >= 2 {
                let talent2_stack = context.attribute.get_value(AttributeName::USER1);
                let value = talent2_stack * 0.4;
                builder.add_extra_bonus("2命「予骄暴者以镣锁」伤害加成",value);
            }
        }
        builder.add_atk_ratio("技能倍率", ratio);

        if s == ChargedTalent1 {
            if context.character_common_data.has_talent1 {
                let punch_bonus = if context.character_common_data.constellation >= 1 {
                    2.0
                } else {
                    0.5
                };
                builder.add_extra_bonus("1命「予行恶者以惩惧」伤害加成",punch_bonus);

                if context.character_common_data.constellation >= 6 {
                    builder.add_extra_critical_damage("6命「予无罪者以念抚」", 0.8);
                    builder.add_extra_critical("6命「予无罪者以念抚」", 0.1);
                }
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Cryo,
            skill_type,
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let talent2_stack = match *config {
            CharacterConfig::Wriothesley { talent2_stack } => talent2_stack,
            _ => 0.0
        };
        Some(Box::new(WriothesleyEffect {
            talent2_stack,
            has_talent2: common_data.has_talent2
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
