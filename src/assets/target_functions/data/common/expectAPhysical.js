import badge from "./badges/sword.png";

function expectA(attribute) {
    let atk = attribute.attack();
    let bonus = attribute.bonus + attribute.physicalBonus + attribute.aBonus;
    let crit = Math.min(attribute.critical, 1);

    let baseDmg = atk * (1 + bonus);
    return (crit * attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "expectAPhysical",
    chs: "期望-物理（平A）",
    description: [
        "使得平A物理伤害的期望伤害最高",
    ],
    formula: "攻击力 * (1 + 伤害加成 + 物理伤害加成 + 平A伤害加成) * (1 + 平A暴击率 * 暴击伤害)",
    tags: [
        "期望",
        "物理",
    ],
    func: expectA,
    "for": "common",
    badge,
}