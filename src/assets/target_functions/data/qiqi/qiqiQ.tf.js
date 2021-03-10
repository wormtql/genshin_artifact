import badge from "@asset/badges/qiqi.png";

import skill from "./skill";

function f(config) {
    let qLevel = config.cArgs.skill3;
    let bonus = skill.q.atk[qLevel - 1];
    let s = skill.q.s[qLevel - 1];

    return function (attribute) {
        let atk = attribute.attack();
        return (atk * bonus + s) * (1 + attribute.cureEffect);
    }
}

export default {
    name: "qiqiQ",
    chs: "七七-救苦渡厄",
    description: [
        "使得符的治疗量最高",
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