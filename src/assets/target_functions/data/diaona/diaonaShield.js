import badge from "./badge.png";

function shieldDiaona(attribute) {
    let hp = attribute.life();
    return (hp * 0.101 + 1097) * (1 + attribute.shield);
}

export default {
    name: "shieldNormal",
    chs: "迪奥娜-猫爪冻冻",
    description: [
        "使得迪奥娜E技能护盾量最大",
        "假设技能6级（不同等级差别不大）"
    ],
    tags: [
        "护盾",
        "迪奥娜",
    ],
    func: shieldDiaona,
    "for": "diaona",
    badge,
}