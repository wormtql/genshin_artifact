use crate::{target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName, target_function::TargetFunctionMetaTrait, target_function_meta::{TargetFunctionMeta, TargetFunctionFor, TargetFunctionMetaImage}}, damage::{DamageContext, SimpleDamageBuilder}, attribute::SimpleAttributeGraph2, character::{characters::Alhaitham, traits::CharacterTrait, skill_config::CharacterSkillConfig, CharacterName}, common::item_config_type::{ItemConfig, ItemConfigType}};
use crate::common::i18n::locale;

pub struct AlhaithamDefaultTargetFunction {
    pub charged_ratio: f64,
    pub e_ratio: f64,
    pub q_ratio: f64,
    pub spread_ratio: f64,
}

impl TargetFunction for AlhaithamDefaultTargetFunction {
    fn get_default_artifact_config(&self, team_config: &crate::team::TeamQuantization) -> crate::artifacts::effect_config::ArtifactEffectConfig {
        Default::default()
    }

    fn get_target_function_opt_config(&self) -> crate::target_functions::target_function_opt_config::TargetFunctionOptConfig {
        unimplemented!()
    }

    fn target(
            &self,
            attribute: &crate::attribute::SimpleAttributeGraph2,
            character: &crate::character::Character<crate::attribute::SimpleAttributeGraph2>,
            weapon: &crate::weapon::Weapon<crate::attribute::SimpleAttributeGraph2>,
            artifacts: &[&crate::artifacts::Artifact],
            enemy: &crate::enemies::Enemy
        ) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            enemy: enemy,
            attribute: &attribute,
        };

        type S = <Alhaitham as CharacterTrait>::DamageEnumType;

        let skill_config = CharacterSkillConfig::Alhaitham { under_e: true };
        let dmg_charged = Alhaitham::damage::<SimpleDamageBuilder>(&context, S::Charged11, &skill_config, None);
        let dmg_e = Alhaitham::damage::<SimpleDamageBuilder>(&context, S::E2, &skill_config, None);
        let dmg_q = Alhaitham::damage::<SimpleDamageBuilder>(&context, S::Q1, &skill_config, None);

        let dmg_normal = dmg_charged.normal.expectation * self.charged_ratio * 2.0
            + dmg_e.normal.expectation * self.e_ratio
            + dmg_q.normal.expectation * self.q_ratio;
        let dmg_spread = dmg_charged.spread.unwrap().expectation * self.charged_ratio * 2.0
            + dmg_e.spread.unwrap().expectation * self.e_ratio
            + dmg_q.spread.unwrap().expectation * self.q_ratio;
        
        dmg_normal + dmg_spread
    }
}

impl TargetFunctionMetaTrait for AlhaithamDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::AlhaithamDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "艾尔海森-诲韬诤言",
            en: "Alhaitham-Admonishing Instruction"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "以重击、1枚光幕攻击伤害、Q单次伤害为基本，乘以相应的次数",
            en: "Charged DMG, 1-Mirror Projection Attack DMG, Single-Instance DMG, multiplied by their counts respectively"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Alhaitham),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [crate::common::item_config_type::ItemConfig]> = Some(&[
        ItemConfig {
            name: "charged_ratio",
            title: locale!(
                zh_cn: "重击次数",
                en: "Charged Hit",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 5.0 },
        },
        ItemConfig {
            name: "e_ratio",
            title: locale!(
                zh_cn: "元素战技次数",
                en: "E Hit"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 5.0 },
        },
        ItemConfig {
            name: "q_ratio",
            title: locale!(
                zh_cn: "元素爆发次数",
                en: "Q Hit",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 1.0 },
        },
        ItemConfig {
            name: "spread_ratio",
            title: locale!(
                zh_cn: "蔓激化比例",
                en: "Spread Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.3 },
        }
    ]);

    fn create(character: &crate::character::character_common_data::CharacterCommonData, weapon: &crate::weapon::weapon_common_data::WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (a, b, c, d) = match *config {
            TargetFunctionConfig::AlhaithamDefault { charged_ratio, e_ratio, q_ratio, spread_ratio } => (charged_ratio, e_ratio, q_ratio, spread_ratio),
            _ => (0.0, 0.0, 0.0, 0.0)
        };
        Box::new(AlhaithamDefaultTargetFunction {
            charged_ratio: a,
            e_ratio: b,
            q_ratio: c,
            spread_ratio: d
        })
    }
}
