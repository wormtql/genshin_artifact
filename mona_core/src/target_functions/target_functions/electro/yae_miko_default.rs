use std::f64::NAN;
use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::electro::yae_miko::YaeMiko;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct YaeMikoDefaultTargetFunction {
    // 充能需求
    pub recharge_requirement: f64,
    // 连招：0 => 仅依靠E和Q；1 => 在0的基础上一直A。
    pub combo: usize,
    // 超激化比例
    pub aggravate_rate: f64,
    // 超绽放比例
    pub hyperbloom_rate: f64,
}

impl YaeMikoDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (
            recharge_requirement,
            combo,
            aggravate_rate,
            hyperbloom_rate,
        ) = match *config {
            TargetFunctionConfig::YaeMikoDefault {
                recharge_requirement,
                combo,
                aggravate_rate,
                hyperbloom_rate,
            } =>
                (
                    recharge_requirement,
                    combo,
                    aggravate_rate,
                    hyperbloom_rate,
                ),
            _ => (0.0, 0, 0.0, 0.0)
        };
        Self {
            recharge_requirement,
            combo,
            aggravate_rate,
            hyperbloom_rate,
        }
    }
}

impl TargetFunction for YaeMikoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .thundersoother(1.0)
            .gilded_dreams(1, 2, 1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <YaeMiko as CharacterTrait>::DamageEnumType;
        let dmg_e = YaeMiko::damage::<SimpleDamageBuilder>(&context, S::E3, &CharacterSkillConfig::NoConfig, None);
        let dmg_a1 = YaeMiko::damage::<SimpleDamageBuilder>(&context, S::Normal1, &CharacterSkillConfig::NoConfig, None);
        let dmg_a2 = YaeMiko::damage::<SimpleDamageBuilder>(&context, S::Normal2, &CharacterSkillConfig::NoConfig, None);
        let dmg_a3 = YaeMiko::damage::<SimpleDamageBuilder>(&context, S::Normal3, &CharacterSkillConfig::NoConfig, None);

        let dmg_e_norm = dmg_e.normal.expectation;
        let dmg_a1_norm = dmg_a1.normal.expectation;
        let dmg_a2_norm = dmg_a2.normal.expectation;
        let dmg_a3_norm = dmg_a3.normal.expectation;

        let dmg_e_aggravate = dmg_e.aggravate.unwrap().expectation;
        let dmg_a1_aggravate = dmg_a1.aggravate.unwrap().expectation;
        let dmg_a2_aggravate = dmg_a2.aggravate.unwrap().expectation;
        let dmg_a3_aggravate = dmg_a3.aggravate.unwrap().expectation;

        let dmg_e_aggravate_bonus = dmg_e_aggravate - dmg_e_norm;
        let dmg_a1_aggravate_bonus = dmg_a1_aggravate - dmg_a1_norm;
        let dmg_a2_aggravate_bonus = dmg_a2_aggravate - dmg_a2_norm;
        let dmg_a3_aggravate_bonus = dmg_a3_aggravate - dmg_a3_norm;

        let mut dmg_hyperbloom = 0.0;
        if self.hyperbloom_rate > 0.0 {
            let transformative = context.transformative();
            dmg_hyperbloom = transformative.hyperbloom;
        }

        // 一轮12s，12下E伤害、6轮A伤害
        let dmg_sum_normal = match self.combo {
            0 => (dmg_e_norm) * 12.0 + (dmg_a1_norm + dmg_a2_norm + dmg_a3_norm) * 0.0,
            1 => (dmg_e_norm) * 12.0 + (dmg_a1_norm + dmg_a2_norm + dmg_a3_norm) * 6.0,
            _ => NAN
        };
        // E的激化率约为1/3（对单）、A的激化率约为1/2
        let dmg_sum_aggravate_bonus = match self.combo {
            0 => (dmg_e_aggravate_bonus) * 12.0 / 3.0 + (dmg_a1_aggravate_bonus + dmg_a2_aggravate_bonus + dmg_a3_aggravate_bonus) * 0.0 / 2.0,
            1 => (dmg_e_aggravate_bonus) * 12.0 / 3.0 + (dmg_a1_aggravate_bonus + dmg_a2_aggravate_bonus + dmg_a3_aggravate_bonus) * 6.0 / 2.0,
            _ => NAN
        };

        // 超绽放伤害冷却为0.5s/2次
        let dmg_sum_hyperbloom = 12.0 / (0.5 / 2.0);

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_requirement);
        r * (dmg_sum_normal +
            dmg_sum_aggravate_bonus * self.aggravate_rate +
            dmg_sum_hyperbloom * self.hyperbloom_rate)
    }
}

impl TargetFunctionMetaTrait for YaeMikoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YaeMikoDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "八重神子-浮世笑百姿",
            en: "Yae-Astute Amusement"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "按照一轮12s：三阶杀生樱12下、普通攻击6×3下计算。由于杀生樱的激化率为1/3、普通攻击的激化率为1/2，在激元素充足的情况下（超激化比例=1），所以一轮杀生樱最大激化4下、普通攻击期望最大9下。超激化比例是根据激元素的充足与否决定实际激化数占最大激化数的比例。超绽放比例是根据草种子的重组与否决定实际绽放的种子数占最大绽放的种子数（0.5s/2个）的比例。",
            en: "DPS Yae Miko"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::YaeMiko),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 },
        },
        ItemConfig {
            name: "combo",
            title: crate::common::i18n::locale!(
                zh_cn: "连招选择",
                en: "Combo",
            ), //连招选择
            config: ItemConfigType::Option { options: "不站场平A,站场平A", default: 0 },
        },
        ItemConfig {
            name: "aggravate_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "超激化比例",
                en: "Aggravate Ratio",
            ), //超激化比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "hyperbloom_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "超绽放比例",
                en: "Hyperbloom Ratio",
            ), //超绽放比例
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(YaeMikoDefaultTargetFunction::new(config))
    }
}
