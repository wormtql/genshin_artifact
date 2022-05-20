use std::collections::HashMap;
use crate::attribute::Attribute;
use crate::buffs::Buff;
use crate::buffs::buffs::{BuffBennettQ, BuffKaedeharaKazuhaTalent2, BuffKujouSaraEOrQ};
use crate::character::CharacterName;
use crate::character::traits::CharacterTrait;
use crate::common::Element;
use crate::target_functions::target_functions::{BennettDefaultTargetFunction, KaedeharaKazuhaDefaultTargetFunction, KujouSaraDefaultTargetFunction, RaidenShogunDefaultTargetFunction};
use crate::target_functions::TargetFunction;
use crate::team::team::Team;
use crate::team_target::team_name::TeamName;
use crate::team_target::team_target_config::TeamTargetFunctionConfig;
use crate::team_target::team_target_function::{TeamTargetFunction, TeamTargetFunctionMetaData, TeamTargetFunctionMetaTrait};

// 雷九万班
pub struct TeamTargetRaidenKujouKazuhaBennett;

impl TeamTargetFunction for TeamTargetRaidenKujouKazuhaBennett {
    fn target(&self, values: &HashMap<usize, f64>) -> f64 {
        let raiden = *values.get(&(CharacterName::RaidenShogun as usize)).unwrap();
        let kujou = *values.get(&(CharacterName::KujouSara as usize)).unwrap();
        let kazuha = *values.get(&(CharacterName::KaedeharaKazuha as usize)).unwrap();
        let bennett = *values.get(&(CharacterName::Bennett as usize)).unwrap();

        (raiden * 0.7 + kujou * 0.15) * kazuha * bennett
    }

    fn get_default_individual_targets(&self) -> HashMap<usize, Box<dyn TargetFunction>> {
        let raiden_target: Box<dyn TargetFunction> = Box::new(RaidenShogunDefaultTargetFunction);
        let kujou_target: Box<dyn TargetFunction> = Box::new(KujouSaraDefaultTargetFunction);
        let kazuha_target: Box<dyn TargetFunction> = Box::new(KaedeharaKazuhaDefaultTargetFunction {
            recharge_demand: 1.6
        });
        let bennett_target: Box<dyn TargetFunction> = Box::new(BennettDefaultTargetFunction);

        let mut ret = HashMap::new();

        ret.insert(CharacterName::RaidenShogun as usize, raiden_target);
        ret.insert(CharacterName::KujouSara as usize, kujou_target);
        ret.insert(CharacterName::KaedeharaKazuha as usize, kazuha_target);
        ret.insert(CharacterName::Bennett as usize, bennett_target);

        ret
    }
}

impl TeamTargetFunctionMetaTrait for TeamTargetRaidenKujouKazuhaBennett {
    const MEMBERS: &'static [CharacterName] = &[CharacterName::RaidenShogun, CharacterName::KujouSara, CharacterName::KaedeharaKazuha, CharacterName::Bennett];

    const META: TeamTargetFunctionMetaData = TeamTargetFunctionMetaData {
        name: TeamName::RaidenKujouKazuhaBennett,
        chs: "雷九万班"
    };

    fn get_default_buffs<A: Attribute>(team: &Team<A>) -> HashMap<usize, Vec<Box<dyn Buff<A>>>> {
        let bennett = team.get_entry_by_name(CharacterName::Bennett).unwrap();
        let bennett_base_atk = bennett.character.common_data.base_atk + bennett.weapon.common_data.base_atk;
        let buff_bennett: Box<dyn Buff<A>> = Box::new(BuffBennettQ {
            base_atk: bennett_base_atk,
            c1: bennett.character.common_data.constellation >= 1,
            skill3: bennett.character.common_data.skill3
        });

        let kujou = team.get_entry_by_name(CharacterName::KujouSara).unwrap();
        let kujou_base_atk = kujou.character.common_data.base_atk + kujou.weapon.common_data.base_atk;
        let buff_kujou: Box<dyn Buff<A>> = Box::new(BuffKujouSaraEOrQ {
            c6: kujou.character.common_data.constellation >= 6,
            base_atk: kujou_base_atk,
            skill2: kujou.character.common_data.skill2
        });

        let buff_kazuha: Box<dyn Buff<A>> = Box::new(BuffKaedeharaKazuhaTalent2 {
            em: 800.0,
            element: Element::Electro
        });

        let mut ret = HashMap::new();
        ret.insert(CharacterName::RaidenShogun as usize, vec![buff_bennett, buff_kujou, buff_kazuha]);

        ret
    }

    fn create<A: Attribute>(_config: &TeamTargetFunctionConfig, _team: &Team<A>) -> Box<dyn TeamTargetFunction> {
        Box::new(TeamTargetRaidenKujouKazuhaBennett)
    }
}
