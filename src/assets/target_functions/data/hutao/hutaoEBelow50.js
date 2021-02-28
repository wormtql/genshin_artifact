import badge from "./fire_slime.png";

import skill from "./skill";

function f(config) {
    let baseAtk = config.character.baseAtk + config.weapon.baseAtk;
    let eLevel = config.cArgs.skill2;
    let atkInc = skill.e.hp[eLevel - 1];
    let hasTalent2 = config.character.hasTalent2;

    return function (attribute) {
        let atkBonus = attribute.life() * atkInc;
        atkBonus = Math.min(atkBonus, 4 * baseAtk);

        let atk = attribute.attack() + atkBonus;
        let bonus = attribute.eBonus + attribute.bonus + attribute.fireBonus + (hasTalent2 ? 0.33 : 0);

        let crit = Math.min(1, attribute.eCritical);

        return atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    };
}

export default {
    name: "hutaoEBelow50",
    chs: "胡桃-蝶引来生-生命值<50%",
    description: [
        "使得胡桃E技能伤害最高", 
        "假设胡桃血量低于50%",
    ],
    tags: [
        "输出",
        "胡桃",
    ],
    func: f,
    "for": "hutao",
    badge,
    needConfig: true,
}