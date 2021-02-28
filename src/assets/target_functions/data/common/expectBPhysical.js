import badge from "./badges/sword.png";

function expectB(attribute) {
    let atk = attribute.attack();
    let bonus = attribute.bonus + attribute.physicalBonus + attribute.bBonus;
    let crit = Math.min(attribute.bCritical, 1);

    let baseDmg = atk * (1 + bonus);
    return (crit * attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "expectBPhysical",
    chs: "期望-物理（重击）",
    description: [
        "使得重击物理伤害的期望伤害最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 物理伤害加成 + 重击伤害加成) * (1 + 重击暴击率 * 暴击伤害)",
    tags: [
        "期望",
        "物理",
    ],
    func: expectB,
    "for": "common",
    badge,
}