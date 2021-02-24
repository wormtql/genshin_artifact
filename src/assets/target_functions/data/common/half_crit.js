import badge from "./badges/sword.png";

function halfCrit(attribute) {
    if (attribute.critical < 0.5) {
        return attribute.critical;
    }

    // let crit = Math.min(1, attribute.critical);
    return attribute.attack() * (1 + 0.5 * attribute.criticalDamage);
}

export default {
    name: "halfCrit",
    chs: "50%暴击",
    description: [
        "如果暴击率达到50%，堆攻击和爆伤，否则堆暴击率",
    ],
    tags: [
        "输出",
    ],
    func: halfCrit,
    "for": "common",
    badge,
}