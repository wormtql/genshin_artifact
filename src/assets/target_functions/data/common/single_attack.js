import badge from "./badges/sword.png";

function attack(attribute) {
    return attribute.attack();
}

export default {
    name: "attack",
    chs: "单值-攻击力",
    description: [
        "使得总攻击力最高"
    ],
    tags: [
        "攻击",
    ],
    func: attack,
    "for": "common",
    badge,
}