import badge from "./badges/badge_attack.png";

function expect(attribute) {
    let critical = Math.min(attribute.critical, 1);
    let atk = attribute.attack();

    let baseDamage = atk * (1 + attribute.aBonus + attribute.physicalBonus + attribute.bonus);
    return critical * (1 + attribute.criticalDamage) * baseDamage + (1 - critical) * baseDamage;
}

export default {
    name: "physicalExpect",
    chs: "物理期望",
    description: [
        "假设只使用平A，且所有平A均为物理伤害",
    ],
    tags: [
        "攻击",
        "通用",
    ],
    func: expect,
    "for": "common",
    badge,
}