import badge from "./badge.png";

function ganyuNormal(attribute) {
    let attack = attribute.attack();
    let crit = Math.min(attribute.bCritical, 1);

    let bonus = attribute.bBonus + attribute.bonus + attribute.iceBonus;
    let baseDmg = attack * (1 + bonus);

    return (crit * attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "ganyuNormal",
    chs: "甘雨-椰羊",
    description: [
        "靠二段蓄力输出",
    ],
    tags: [
        "输出",
        "甘雨",
    ],
    func: ganyuNormal,
    "for": "ganyu",
    badge,
}