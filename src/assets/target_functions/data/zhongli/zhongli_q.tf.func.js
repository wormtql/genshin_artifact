import skill from "./skill";

function zhongliQ(config) {
    let qLevel = config.cArgs.skill3;
    let dmg = skill.q.dmg[qLevel - 1];
    let hasTalent2 = config.character.hasTalent2;

    return function (attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute.rockBonus + attribute.qBonus;
        let extraDmg = attribute.life() * 0.33;
        let baseDmg = dmg * atk * (1 + bonus) + (hasTalent2 ? extraDmg : 0);
        let crit = Math.min(1, attribute.qCritical);

        return (1 + attribute.criticalDamage * crit) * baseDmg;
    };
}

export default {
    name: "zhongliQ",
    func: zhongliQ,
    needConfig: true,
}