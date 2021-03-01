import badge from "./badge.png";
import config from "./HutaoConfig";

import skill from "./skill";

function f(config) {
    let baseAtk = config.character.baseAtk + config.weapon.baseAtk;
    let eLevel = config.cArgs.skill2;
    let atkInc = skill.e.hp[eLevel - 1];

    let hasTalent2 = config.character.hasTalent2;

    let hpBelow50 = config.tArgs.hpBelow50;
    let talentBonus = (hasTalent2 && hpBelow50) ? 0.33 : 0;

    return function (attribute) {
        let atkBonus = attribute.life() * atkInc;
        atkBonus = Math.min(atkBonus, 4 * baseAtk);

        let atk = attribute.attack() + atkBonus;
        let bonus = attribute.aBonus + attribute.bonus + attribute.fireBonus +talentBonus;

        let crit = Math.min(1, attribute.critical);

        return atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    };
}

export default {
    name: "hutaoE",
    chs: "胡桃-蝶引来生",
    description: [
        "使得胡桃E技能后的攻击伤害最高",
    ],
    tags: [
        "输出",
        "胡桃",
    ],
    func: f,
    "for": "hutao",
    badge,
    needConfig: true,
    config,
}