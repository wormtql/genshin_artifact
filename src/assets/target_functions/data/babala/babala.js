import badge from "./badge.png";

import skill from "./skill";

function f(config) {
    let qLevel = config.cArgs.skill3 || 6;
    let bonus = skill.q.hp[qLevel - 1];
    let s = skill.q.s[qLevel - 1];

    return function (attribute) {
        let hp = attribute.life();
        return (hp * bonus + s) * (1 + attribute.cureEffect);
    };
}

export default {
    name: "babalaQ",
    chs: "芭芭拉-闪耀奇迹♪",
    description: [
        "使得芭芭拉Q技能治疗效果最好",
    ],
    tags: [
        "治疗",
        "芭芭拉",
    ],
    func: f,
    "for": "babala",
    badge,
    needConfig: true,
}