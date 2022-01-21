use crate::artifacts::ArtifactSetName;
use crate::attribute::Attribute;
use crate::common::{Element, SkillType, StatName};
use crate::damage::DamageBuilder;
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::target_functions::target_function::GetTargetFunctionOptConfig;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;

pub struct MaxTargetFunction {
    pub element: Element,
    pub skill_type: SkillType,
}

impl MaxTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> MaxTargetFunction {
        match config {
            TargetFunctionConfig::MaxConfig { element, skill_type } => {
                MaxTargetFunction {
                    element: *element,
                    skill_type: *skill_type
                }
            },
            _ => panic!("wrong target function config, expecting MaxConfig")
        }
    }
}

impl<T: Attribute> TargetFunction<T> for MaxTargetFunction {
    fn target(&self, attribute: &T, enemy: &Enemy) -> f64 {
        DamageBuilder::new(3.0, 0.0, 0.0).damage(
            attribute, enemy, self.element, self.skill_type, 90
        ).critical
    }
}

impl GetTargetFunctionOptConfig for MaxTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        let mut stat_name1 = StatName::stat_name_bonus_from_element(self.element);

        let mut set_names = match self.element {
            Element::Electro => vec![
                ArtifactSetName::ThunderingFury,
                ArtifactSetName::Thundersoother,
            ],
            Element::Pyro => vec![
                ArtifactSetName::Lavawalker,
                ArtifactSetName::CrimsonWitchOfFlames
            ],
            Element::Hydro => vec![
                ArtifactSetName::HeartOfDepth
            ],
            Element::Anemo => vec![
                ArtifactSetName::ViridescentVenerer
            ],
            Element::Geo => vec![
                ArtifactSetName::ArchaicPetra,
                ArtifactSetName::HuskOfOpulentDreams
            ],
            Element::Cryo => vec![
                ArtifactSetName::BlizzardStrayer
            ],
            Element::Physical => vec![
                ArtifactSetName::BloodstainedChivalry,
                ArtifactSetName::PaleFlame,
            ],
            _ => unreachable!(),
        };
        set_names.extend(vec![
            ArtifactSetName::GladiatorsFinale,
            ArtifactSetName::ShimenawasReminiscence,
            ArtifactSetName::EmblemOfSeveredFate,
            ArtifactSetName::RetracingBolide,
        ]);

        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.5,
            elemental_mastery: 0.0,
            critical: 0.0,
            critical_damage: 1.0,
            bonus_electro: if self.element == Element::Electro { 2.0 } else { 0.0 },
            bonus_pyro: if self.element == Element::Pyro { 2.0 } else { 0.0 },
            bonus_hydro: if self.element == Element::Hydro { 2.0 } else { 0.0 },
            bonus_anemo: if self.element == Element::Anemo { 2.0 } else { 0.0 },
            bonus_cryo: if self.element == Element::Cryo { 2.0 } else { 0.0 },
            bonus_geo: if self.element == Element::Geo { 2.0 } else { 0.0 },
            bonus_dendro: if self.element == Element::Dendro { 2.0 } else { 0.0 },
            bonus_physical: if self.element == Element::Physical { 2.0 } else { 0.0 },

            sand_main_stats: vec![StatName::ATKPercentage],
            goblet_main_stats: vec![
                StatName::ATKPercentage,
                stat_name1,
            ],
            head_main_stats: vec![
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],

            set_names: Some(set_names),
        }
    }
}