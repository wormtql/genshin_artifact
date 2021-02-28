import badge from "./badge.png";

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
    chs: "钟离-天动万象",
    description: [
        "使得钟离Q技能的伤害期望最高",
        "若等级不足会忽略第二天赋",
    ],
    tags: [
        "钟离",
    ],
    func: zhongliQ,
    "for": "zhongli",
    badge,
    needConfig: true,
}