import badge from "./badge.png";

import skill from "./skill";

function shieldDiaona(config) {
    let eLevel = config.cArgs.skill2;
    let bonus = skill.e.hp[eLevel - 1];
    let s = skill.e.s[eLevel - 1];

    return function (attribute) {
        let hp = attribute.life();
        return (hp * bonus + s) * (1 + attribute.shield);
    }
}

export default {
    name: "diaonaShield",
    chs: "迪奥娜-猫爪冻冻",
    description: [
        "使得迪奥娜E技能护盾量最大",
    ],
    tags: [
        "护盾",
        "迪奥娜",
    ],
    func: shieldDiaona,
    "for": "diaona",
    badge,
    needConfig: true,
}