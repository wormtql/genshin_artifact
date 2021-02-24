import badge from "./badge.png";

function normalDiaona(attribute) {
    let hp = attribute.life();
    return (hp * 0.0747 + 813) * (1 + attribute.cureEffect);
}

export default {
    name: "diaonaNormal",
    chs: "迪奥娜-猫尾特调",
    description: [
        "使得迪奥娜Q技能治疗效果最好",
        "假设技能6级（不同等级差别不大）"
    ],
    tags: [
        "治疗",
        "迪奥娜",
    ],
    func: normalDiaona,
    "for": "diaona",
    badge,
}