import badge from "@asset/badges/nuoaier.png";

import skill from "./skill";

function f(config) {
    let qLevel = config.cArgs.skill3 || 6;
    let dmg = skill.q.dmg2[qLevel - 1];
    let atkBonus = skill.q.atkBonus[qLevel - 1];

    const isConste6 = config.cArgs.constellation === 6;
    if (isConste6) {
        atkBonus += 0.5;
    }

    return function (attribute) {
        let def = attribute.defend();
        let atk = attribute.attack() + def * atkBonus;

        let crit = Math.min(1, attribute.critical);

        let bonus = attribute.bonus + attribute.rockBonus + attribute.aBonus;
        
        return dmg * atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    };
}

export default {
    name: "nuoaierNormal",
    chs: "诺艾尔-原石的重量，令人安息",
    description: [
        "使得诺艾尔大招伤害最高",
        "技能等级对结果理论上有影响",
        "6命的加成会考虑进去，无须担心6命计算不准确",
    ],
    tags: [
        "输出",
        "诺艾尔",
    ],
    func: f,
    "for": "nuoaier",
    badge,
    needConfig: true,
}