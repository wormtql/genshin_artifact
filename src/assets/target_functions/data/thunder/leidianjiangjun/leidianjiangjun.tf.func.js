import { charactersData } from "@asset/character";


let skill = charactersData["leidianjiangjun"].skill;

function f(config) {
    let underE = config.tArgs.underE;
    let skill2 = config.cArgs.skill2;
    let bonusFromE = skill.e.bonus[skill2 - 1] * 90;

    return function (attribute) {
        let atk = attribute.attack();
        
        let bonus = attribute.bonus + attribute.thunderBonus + attribute.qBonus;
        if (underE) {
            bonus += bonusFromE;
        }
        let crit = Math.min(1, attribute.qCritical);
        let cd = attribute.criticalDamage;

        return atk * (1 + crit * cd) * (1 + bonus);
    }
}

export default {
    name: "leidianjiangjunNormal",
    func: f,
    needConfig: true,
    needContext: false,
}