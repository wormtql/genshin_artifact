import { charactersData } from "@asset/character";


let skill = charactersData["zaoyou"].skill;

function f(config) {
    let qLevel = config.cArgs.skill3;
    let ratio = skill.q.dmg2[qLevel - 1];
    let isconste6 = config.cArgs.constellation;

    if (!isconste6) {
        return function (attribute) {
            let atk = attribute.attack();
            let crit = Math.min(attribute.qCritical, 1);
            let cd = attribute.criticalDamage;
            let bonus = attribute.qBonus + attribute.windBonus + attribute.bonus;

            return ratio * atk * (1 + crit * cd) * (1 + bonus);
        }
    } else {
        return function (attribute) {
            let atk = attribute.attack();
            let crit = Math.min(attribute.qCritical, 1);
            let cd = attribute.criticalDamage;
            let bonus = attribute.qBonus + attribute.windBonus + attribute.bonus;

            let extra = Math.min(4, attribute.elementalMastery * 0.002);
            return (ratio + extra) * atk * (1 + crit * cd) * (1 + bonus);
        }
    }
}


export default {
    name: "zaoyou6",
    func: f,
    needConfig: true,
}