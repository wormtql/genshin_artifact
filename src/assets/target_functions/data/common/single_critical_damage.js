import badge from "./badges/sword.png";

function cd(attribute) {
    return attribute.criticalDamage;
}

export default {
    name: "criticalDamage",
    chs: "单值-暴击伤害",
    description: [
        "使得总暴击伤害最高"
    ],
    tags: [
        "暴击",
        "输出",
    ],
    func: cd,
    "for": "common",
    badge,
}