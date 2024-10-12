use crate::common::{StatName, ChangeAttribute};
use crate::attribute::Attribute;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum CharacterSubStatFamily {
    ATK240,
    ATK288,

    Bonus240(StatName),
    Bonus288(StatName),
    Bonus300(StatName),

    CriticalDamage384,

    CriticalRate192,

    HealingBonus222,

    DEF300,
    DEF360,

    ElementalMastery96,
    ElementalMastery115,

    HP240,
    HP288,

    Recharge267,
    Recharge320,
}


pub fn get_value_array(family: CharacterSubStatFamily) -> [f64; 5] {
    match family {
        CharacterSubStatFamily::ATK240 => [0.06, 0.12, 0.12, 0.18, 0.24],
        CharacterSubStatFamily::ATK288 => [0.072, 0.144, 0.144, 0.216, 0.288],
        CharacterSubStatFamily::Bonus240(_) => [0.06, 0.12, 0.12, 0.18, 0.24],
        CharacterSubStatFamily::Bonus288(_) => [0.072, 0.144, 0.144, 0.216, 0.288],
        CharacterSubStatFamily::Bonus300(_) => [0.75, 0.15, 0.15, 0.225, 0.3],
        CharacterSubStatFamily::CriticalDamage384 => [0.096, 0.192, 0.192, 0.288, 0.384],
        CharacterSubStatFamily::CriticalRate192 => [0.048, 0.096, 0.096, 0.144, 0.192],
        CharacterSubStatFamily::HealingBonus222 => [0.055, 0.111, 0.111, 0.166, 0.222],
        CharacterSubStatFamily::DEF300 => [0.075, 0.15, 0.15, 0.225, 0.3],
        CharacterSubStatFamily::DEF360 => [0.09, 0.18, 0.18, 0.27, 0.36],
        CharacterSubStatFamily::ElementalMastery96 => [24.0, 48.0, 48.0, 72.0, 96.0],
        CharacterSubStatFamily::ElementalMastery115 => [29.0, 58.0, 58.0, 86.0, 115.0],
        CharacterSubStatFamily::HP240 => [0.06, 0.12, 0.12, 0.18, 0.24],
        CharacterSubStatFamily::HP288 => [0.072, 0.144, 0.144, 0.216, 0.288],
        CharacterSubStatFamily::Recharge267 => [0.067, 0.133, 0.133, 0.2, 0.267],
        CharacterSubStatFamily::Recharge320 => [0.08, 0.16, 0.16, 0.24, 0.32],
    }
}

pub fn get_stat_name_from_family(family: CharacterSubStatFamily) -> StatName {
    use CharacterSubStatFamily::*;
    match family {
        ATK240 | ATK288 => StatName::ATKPercentage,
        Bonus240(x) => x,
        Bonus288(x) => x,
        Bonus300(x) => x,
        CriticalDamage384 => StatName::CriticalDamage,
        CriticalRate192 => StatName::CriticalRate,
        HealingBonus222 => StatName::HealingBonus,
        DEF300 => StatName::DEFPercentage,
        DEF360 => StatName::DEFPercentage,
        ElementalMastery96 => StatName::ElementalMastery,
        ElementalMastery115 => StatName::ElementalMastery,
        HP240 | HP288 => StatName::HPPercentage,
        Recharge267 => StatName::Recharge,
        Recharge320 => StatName::Recharge,
    }
}


pub struct CharacterSubStat {
    pub value: f64,
    pub stat_name: StatName,
    pub attribute_key: String,
}

impl CharacterSubStat {
    pub fn new(family: CharacterSubStatFamily, level: usize, ascend: bool) -> CharacterSubStat {
        let array = get_value_array(family);

        let value = if level < 40 || (level == 40 && !ascend) {
            0.0
        } else if level < 50 || (level == 50 && !ascend) {
            array[0]
        } else if level < 60 || (level == 60 && !ascend) {
            array[1]
        } else if level < 70 || (level == 70 && !ascend) {
            array[2]
        } else if level < 80 || (level == 80 && !ascend) {
            array[3]
        } else {
            array[4]
        };

        let stat_name = get_stat_name_from_family(family);

        CharacterSubStat {
            value, stat_name,
            attribute_key: String::from("角色副属性")
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for CharacterSubStat {
    fn change_attribute(&self, attribute: &mut T) {
        self.stat_name.apply(attribute, self.attribute_key.as_str(), self.value);
    }
}