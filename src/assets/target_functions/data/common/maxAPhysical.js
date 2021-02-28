import badge from "./badges/sword.png";

function maxA(attribute) {
    let atk = attribute.attack();
    let bonus = attribute.bonus + attribute.physicalBonus + attribute.aBonus;

    let baseDmg = atk * (1 + bonus);
    return (attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "maxAPhysical",
    chs: "上限-物理（平A）",
    description: [
        "使得平A物理伤害的上限最高",
    ],
    formula: "攻击力 * (1 + 伤害加成 + 物理伤害加成 + 平A伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
        "物理",
    ],
    func: maxA,
    "for": "common",
    badge,
}