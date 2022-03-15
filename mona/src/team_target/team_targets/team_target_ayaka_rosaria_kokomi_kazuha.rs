use std::collections::HashMap;
use crate::attribute::Attribute;
use crate::buffs::Buff;
use crate::buffs::buffs::{BuffATKPercentage, BuffCritical, BuffCustomBonus};
use crate::character::CharacterName;
use crate::target_functions::target_functions::{KaedeharaKazuhaDefaultTargetFunction, KamisatoAyakaDefaultTargetFunction, RosariaDefaultTargetFunction, SangonomiyaKokomiDefaultTargetFunction};
use crate::target_functions::TargetFunction;
use crate::team::team::Team;
use crate::team_target::team_name::TeamName;
use crate::team_target::team_target_config::TeamTargetFunctionConfig;
use crate::team_target::team_target_function::{TeamTargetFunction, TeamTargetFunctionMetaData, TeamTargetFunctionMetaTrait};

pub struct TeamTargetAyakaRosariaKokomiKazuha;

impl TeamTargetFunction for TeamTargetAyakaRosariaKokomiKazuha {
    fn target(&self, values: &HashMap<usize, f64>) -> f64 {
        let ayaka = *values.get(&(CharacterName::KamisatoAyaka as usize)).unwrap();
        let rosaria = *values.get(&(CharacterName::Rosaria as usize)).unwrap();
        let kokomi = *values.get(&(CharacterName::SangonomiyaKokomi as usize)).unwrap();
        let kazuha = *values.get(&(CharacterName::KaedeharaKazuha as usize)).unwrap();

        return ayaka * 0.8 + kokomi * 0.05 + kazuha * 0.1 + rosaria * 0.05
    }

    fn get_default_individual_targets(&self) -> HashMap<usize, Box<dyn TargetFunction>> {
        let mut results: HashMap<usize, Box<dyn TargetFunction>> = HashMap::new();

        results.insert(CharacterName::KamisatoAyaka as usize, Box::new(KamisatoAyakaDefaultTargetFunction));
        results.insert(CharacterName::Rosaria as usize, Box::new(RosariaDefaultTargetFunction {
            other_atk_bonus_percentage: 1.5,
            other_critical: 0.7,
            other_critical_damage: 2.0
        }));
        results.insert(CharacterName::SangonomiyaKokomi as usize, Box::new(SangonomiyaKokomiDefaultTargetFunction));
        results.insert(CharacterName::KaedeharaKazuha as usize, Box::new(KaedeharaKazuhaDefaultTargetFunction {
            recharge_demand: 1.4
        }));

        results
    }
}

impl TeamTargetFunctionMetaTrait for TeamTargetAyakaRosariaKokomiKazuha {
    const MEMBERS: &'static [CharacterName] = &[CharacterName::KamisatoAyaka, CharacterName::Rosaria, CharacterName::SangonomiyaKokomi, CharacterName::KaedeharaKazuha];

    const META: TeamTargetFunctionMetaData = TeamTargetFunctionMetaData { name: TeamName::AyakaRosariaKokomiKazuha, chs: "神罗心万" };

    fn get_default_buffs<A: Attribute>(_team: &Team<A>) -> HashMap<usize, Vec<Box<dyn Buff<A>>>> {
        let mut results: HashMap<usize, Vec<Box<dyn Buff<A>>>> = HashMap::new();
        
        results.insert(CharacterName::KamisatoAyaka as usize, vec![
            Box::new(BuffATKPercentage {
                value: 0.5
            }),
            Box::new(BuffCustomBonus {
                value: 0.5
            }),
            Box::new(BuffCritical {
                value: 0.12
            })
        ]);

        results
    }

    fn create<A: Attribute>(_config: &TeamTargetFunctionConfig, _team: &Team<A>) -> Box<dyn TeamTargetFunction> {
        Box::new(TeamTargetAyakaRosariaKokomiKazuha)
    }
}
