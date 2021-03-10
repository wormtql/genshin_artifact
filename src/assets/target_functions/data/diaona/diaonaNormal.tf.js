import badge from "@asset/badges/diaona.png";

import skill from "./skill";

function normalDiaona(config) {
    let qLevel = config.cArgs.skill3;
    let bonus = skill.q.hp[qLevel - 1];
    let s = skill.q.s[qLevel - 1];

    return function (attribute, context) {
        let hp = attribute.life();
        let isMB4 = context.artifactSet.maidenBeloved >= 4;
        return (hp * bonus + s) * (1 + attribute.cureEffect + (isMB4 ? 0.2 : 0));
    }
}

export default {
    name: "diaonaNormal",
    chs: "迪奥娜-猫尾特调",
    description: [
        "使得迪奥娜Q技能治疗效果最好",
        "少女4计全buff",
    ],
    tags: [
        "治疗",
        "迪奥娜",
    ],
    func: normalDiaona,
    "for": "diaona",
    badge,
    needConfig: true,
    needContext: true,
}