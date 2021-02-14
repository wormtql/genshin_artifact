import badge from "./badges/sword.png";

function critical(attribute) {
    if (attribute.critical >= 1) {
        return attribute.criticalDamage * 100;
    }
    return attribute.critical;
}

export default {
    name: "critical",
    chs: "单值-暴击率",
    description: [
        "使得总暴击率最高",
        "暴击率超过100%的情况下将使得暴击伤害最高",
    ],
    tags: [
        "暴击",
        "输出",
    ],
    func: critical,
    "for": "common",
    badge,
}