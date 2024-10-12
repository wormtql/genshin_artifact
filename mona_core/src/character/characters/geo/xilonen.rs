use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::kachina::KachinaDamageEnum;
use crate::character::characters::kachina::KachinaDamageEnum::{Normal1, Normal21, Normal22, Normal3, Normal4};
use crate::character::characters::navia::NaviaDamageEnum;
use crate::character::characters::navia::NaviaDamageEnum::{Plunging1, Plunging2, Plunging3};
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type, declare_skill_type};
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

declare_skill_type!(XilonenSkillType,
    normal_dmg1,
    normal_dmg2,
    normal_dmg3,
    charged_dmg,
    plunging_dmg1,
    plunging_dmg2,
    plunging_dmg3,
    normal_dmg4,
    normal_dmg5,
    normal_dmg6,
    normal_dmg7,

    e_dmg1,
    e_res,

    q_dmg1,
    q_heal1,
    q_heal1_fixed,
    q_dmg2,
);

pub const XILONEN_SKILL: XilonenSkillType = XilonenSkillType {
    normal_dmg1: [0.5179, 0.5601, 0.6022, 0.6625, 0.7046, 0.7528, 0.819, 0.8853, 0.9515, 1.0238, 1.0961, 1.1683, 1.2406, 1.3129, 1.3851],
    normal_dmg2: [0.2737, 0.296, 0.3183, 0.3501, 0.3724, 0.3979, 0.4329, 0.4679, 0.5029, 0.5411, 0.5793, 0.6175, 0.6557, 0.6939, 0.7321],
    normal_dmg3: [0.7295, 0.7889, 0.8483, 0.9331, 0.9925, 1.0603, 1.1536, 1.2469, 1.3402, 1.442, 1.5438, 1.6456, 1.7474, 1.8492, 1.951],
    charged_dmg: [0.9133, 0.9877, 1.062, 1.1682, 1.2425, 1.3275, 1.4443, 1.5611, 1.678, 1.8054, 1.9328, 2.0603, 2.1877, 2.3152, 2.4426],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    normal_dmg4: [0.5602, 0.6058, 0.6514, 0.7166, 0.7622, 0.8143, 0.8859, 0.9576, 1.0292, 1.1074, 1.1856, 1.2638, 1.3419, 1.4201, 1.4983],
    normal_dmg5: [0.5505, 0.5953, 0.6401, 0.7041, 0.7489, 0.8001, 0.8705, 0.9409, 1.0113, 1.0882, 1.165, 1.2418, 1.3186, 1.3954, 1.4722],
    normal_dmg6: [0.6582, 0.7117, 0.7653, 0.8418, 0.8954, 0.9566, 1.0408, 1.125, 1.2092, 1.301, 1.3928, 1.4847, 1.5765, 1.6684, 1.7602],
    normal_dmg7: [0.8603, 0.9303, 1.0003, 1.1004, 1.1704, 1.2504, 1.3604, 1.4705, 1.5805, 1.7005, 1.8206, 1.9406, 2.0607, 2.1807, 2.3007],
    e_dmg1: [1.792, 1.9264, 2.0608, 2.24, 2.3744, 2.5088, 2.688, 2.8672, 3.0464, 3.2256, 3.4048, 3.584, 3.808, 4.032, 4.256],
    e_res: [0.09, 0.12, 0.15, 0.18, 0.21, 0.24, 0.27, 0.3, 0.33, 0.36, 0.39, 0.42, 0.45, 0.48, 0.51],
    q_dmg1: [2.8128, 3.0238, 3.2347, 3.516, 3.727, 3.9379, 4.2192, 4.5005, 4.7818, 5.063, 5.3443, 5.6256, 5.9772, 6.3288, 6.6804],
    q_heal1: [1.04, 1.118, 1.196, 1.3, 1.378, 1.456, 1.56, 1.664, 1.768, 1.872, 1.976, 2.08, 2.21, 2.34, 2.47],
    q_heal1_fixed: [500.74, 550.82, 605.07, 663.5, 726.1, 792.88, 863.82, 938.94, 1018.24, 1101.71, 1189.35, 1281.16, 1377.15, 1477.31, 1581.65],
    q_dmg2: [2.8128, 3.0238, 3.2347, 3.516, 3.727, 3.9379, 4.2192, 4.5005, 4.7818, 5.063, 5.3443, 5.6256, 5.9772, 6.3288, 6.6804],
};

damage_enum!(
    XilonenDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    Plunging1
    Plunging2
    Plunging3
    Normal4
    Normal5
    Normal6
    Normal7
    E1
    Q1
    QHeal
    Q2
);

impl XilonenDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use XilonenDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Normal6 | Normal7 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | QHeal | Q2 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self, nightsoul: bool) -> Element {
        use XilonenDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged => Element::Physical,
            Plunging1 | Plunging2 | Plunging3 => if nightsoul { Element::Geo } else { Element::Physical },
            Normal4 | Normal5 | Normal6 | Normal7 => Element::Geo,
            E1 | Q1 | QHeal | Q2 => Element::Geo,
        }
    }

    pub fn is_def_ratio(&self) -> bool {
        use XilonenDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged => false,
            _ => true
        }
    }
}

struct XilonenEffect {
    pub sampler_geo: bool,
    pub sampler_cryo: bool,
    pub sampler_pyro: bool,
    pub sampler_hydro: bool,
    pub sampler_electro: bool,

    pub talent1_rate: f64,
    pub talent2_rate: f64,
}

impl XilonenEffect {
    pub fn from_float(f: f64) -> Self {
        let u = f as usize;

        let sampler_geo = u >> 4;
        let sampler_cryo = (u >> 3) & 1;
        let sampler_pyro = (u >> 2) & 1;
        let sampler_hydro = (u >> 1) & 1;
        let sampler_electro = u & 1;

        Self {
            sampler_geo: sampler_geo == 1,
            sampler_cryo: sampler_cryo == 1,
            sampler_pyro: sampler_pyro == 1,
            sampler_hydro: sampler_hydro == 1,
            sampler_electro: sampler_electro == 1,
            talent1_rate: 0.0,
            talent2_rate: 0.0,
        }
    }

    pub fn converted_count(&self) -> usize {
        let mut result = 0;
        if self.sampler_cryo {
            result += 1;
        }
        if self.sampler_pyro {
            result += 1;
        }
        if self.sampler_hydro {
            result += 1;
        }
        if self.sampler_electro {
            result += 1;
        }

        result
    }
}

impl<A: Attribute> ChangeAttribute<A> for XilonenEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // let encoding = ((self.sampler_geo as usize) << 4) | ((self.sampler_cryo as usize) << 3) | ((self.sampler_pyro as usize) << 2)
        //     | ((self.sampler_hydro as usize) << 1) | (self.sampler_electro as usize);
        //
        // attribute.set_value_by(AttributeName::USER1, "", encoding as f64);

        let converted_count = self.converted_count();
        if converted_count < 2 {
            attribute.set_value_by(AttributeName::BonusNormalAttack, "天赋「四境四象回声」", 0.3 * self.talent1_rate);
        }

        attribute.add_def_percentage("天赋「便携铠装护层」", 0.2 * self.talent2_rate);
    }
}

pub struct Xilonen;

impl CharacterTrait for Xilonen {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Xilonen,
        internal_name: "Xilonen",
        name_locale: locale!(
            zh_cn: "希诺宁",
            en: "Xilonen"
        ),
        element: Element::Geo,
        hp: [966, 2505, 3333, 4987, 5576, 6415, 7199, 8047, 8636, 9491, 10079, 10945, 11533, 12405],
        atk: [21, 56, 74, 111, 124, 142, 160, 178, 191, 210, 223, 243, 256, 275],
        def: [72, 188, 250, 374, 418, 481, 540, 603, 647, 712, 756, 820, 865, 930],
        sub_stat: CharacterSubStatFamily::DEF360,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·锐锋攫猎",
            en: "Normal Attack: Ehecatl's Roar"
        ),
        skill_name2: locale!(
            zh_cn: "音火锻淬",
            en: "Yohual's Scratch"
        ),
        skill_name3: locale!(
            zh_cn: "豹烈律动！",
            en: "Ocelotlicue Point!"
        ),
    };
    type SkillType = XilonenSkillType;
    const SKILL: Self::SkillType = XILONEN_SKILL;
    type DamageEnumType = XilonenDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            XilonenDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
            Normal4 locale!(zh_cn: "刃轮巡猎一段伤害", en: "Blade Roller 1-Hit DMG")
            Normal5 locale!(zh_cn: "刃轮巡猎二段伤害", en: "Blade Roller 2-Hit DMG")
            Normal6 locale!(zh_cn: "刃轮巡猎三段伤害", en: "Blade Roller 3-Hit DMG")
            Normal7 locale!(zh_cn: "刃轮巡猎四段伤害", en: "Blade Roller 4-Hit DMG")
        ),
        skill2: skill_map!(
            XilonenDamageEnum
            E1 locale!(zh_cn: "突进伤害", en: "Rush DMG")
        ),
        skill3: skill_map!(
            XilonenDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QHeal locale!(zh_cn: "持续治疗量", en: "Continuous Healing")
            Q2 locale!(zh_cn: "追加节拍伤害", en: "Follow-Up Beat DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "sampler_geo",
            title: locale!(zh_cn: "源音采样-岩", en: "Sampler Geo"),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "sampler_cryo",
            title: locale!(zh_cn: "源音采样-冰", en: "Sampler Cryo"),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "sampler_pyro",
            title: locale!(zh_cn: "源音采样-火", en: "Sampler Pyro"),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "sampler_hydro",
            title: locale!(zh_cn: "源音采样-水", en: "Sampler Hydro"),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "sampler_electro",
            title: locale!(zh_cn: "源音采样-雷", en: "Sampler Electro"),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "talent1_rate",
            title: locale!(zh_cn: "天赋1比例", en: "Talent 1 Ratio"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "talent2_rate",
            title: locale!(zh_cn: "天赋2比例", en: "Talent 2 Ratio"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "nightsoul",
            title: locale!(zh_cn: "夜魂状态·刃轮巡猎", en: "Nightsoul's Blessing: Blade Roller"),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: XilonenDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use XilonenDamageEnum::*;

        if s == QHeal {
            let ratio = XILONEN_SKILL.q_heal1[s3];
            let fixed = XILONEN_SKILL.q_heal1_fixed[s3];

            let mut builder = D::new();
            builder.add_def_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            return builder.heal(&context.attribute);
        }

        let ratio = match s {
            Normal1 => XILONEN_SKILL.normal_dmg1[s1],
            Normal2 => XILONEN_SKILL.normal_dmg2[s1],
            Normal3 => XILONEN_SKILL.normal_dmg3[s1],
            Charged => XILONEN_SKILL.charged_dmg[s1],
            Plunging1 => XILONEN_SKILL.plunging_dmg1[s1],
            Plunging2 => XILONEN_SKILL.plunging_dmg2[s1],
            Plunging3 => XILONEN_SKILL.plunging_dmg3[s1],
            Normal4 => XILONEN_SKILL.normal_dmg4[s1],
            Normal5 => XILONEN_SKILL.normal_dmg5[s1],
            Normal6 => XILONEN_SKILL.normal_dmg6[s1],
            Normal7 => XILONEN_SKILL.normal_dmg7[s1],
            E1 => XILONEN_SKILL.e_dmg1[s2],
            Q1 => XILONEN_SKILL.q_dmg1[s3],
            Q2 => XILONEN_SKILL.q_dmg2[s3],
            _ => 0.0
        };
        let is_def_ratio = s.is_def_ratio();

        let mut builder = D::new();
        if is_def_ratio {
            builder.add_def_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        let nightsoul = match *config {
            CharacterSkillConfig::Xilonen { nightsoul } => nightsoul,
            _ => false
        };

        if nightsoul && context.character_common_data.constellation >= 6 {
            if s == Normal4 || s == Normal5 || s == Normal6 || s == Normal7 ||
                s == Plunging1 || s == Plunging2 || s == Plunging3 {
                builder.add_def_ratio("C6「献予永夜的狂欢舞」", 3.0);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(nightsoul),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Xilonen { sampler_geo, sampler_cryo, sampler_pyro, sampler_hydro, sampler_electro, talent1_rate, talent2_rate } =>
                Some(Box::new(XilonenEffect {
                    sampler_geo, sampler_cryo, sampler_pyro, sampler_hydro, sampler_electro, talent1_rate, talent2_rate
                })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
