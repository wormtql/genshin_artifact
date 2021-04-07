import skill from "./skill";

function f(config) {
    let qLevel = config.cArgs.skill3;
    let b = skill.q.bonus[qLevel - 1];

    return function xiaoQ(attribute) {
        let attack = attribute.attack();
        let crit = Math.min(attribute.airCritical, 1);
    
        let bonus = attribute.airBonus + attribute.bonus + attribute.windBonus + b;
    
        return attack * (1 + bonus) * (crit * attribute.criticalDamage + 1);
    }
}


export default {
    name: "xiaoQ",
    func: f,
    needConfig: true,
}