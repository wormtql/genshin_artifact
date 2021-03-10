import badge from "@asset/badges/qiqi.png";

import skill from "./skill";

function f(config) {
    let eLevel = config.cArgs.skill2;
    let bonus = skill.e.atk2[eLevel - 1];
    let s = skill.e.s2[eLevel - 1];

    return function (attribute) {
        let atk = attribute.attack();
        return (atk * bonus + s) * (1 + attribute.cureEffect);
    }
}

export default {
    name: "qiqiE2",
    chs: "七七-寒病鬼差-持续治疗",
    description: [
        "使得七七E技能下持续治疗量最高",
    ],
    tags: [
        "治疗",
        "七七",
    ],
    func: f,
    "for": "qiqi",
    badge,
    needConfig: true,
}