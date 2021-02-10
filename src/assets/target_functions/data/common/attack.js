import badge from "./badges/badge_attack.png";

function attack(attribute) {
    return attribute.attack();
}

export default {
    name: "attack",
    chs: "攻击力",
    description: [
        "使得总攻击力最高"
    ],
    tags: [
        "攻击",
        "通用",
    ],
    func: attack,
    "for": "common",
    badge,
}