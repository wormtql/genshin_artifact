import badge from "./badges/sword.png";

function maxB(attribute) {
    let atk = attribute.attack();
    let bonus = attribute.bonus + attribute.physicalBonus + attribute.bBonus;

    let baseDmg = atk * (1 + bonus);
    return (attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "maxBPhysical",
    chs: "上限-物理（重击）",
    description: [
        "使得重击物理伤害的上限最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 物理伤害加成 + 重击伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
        "物理",
    ],
    func: maxB,
    "for": "common",
    badge,
}