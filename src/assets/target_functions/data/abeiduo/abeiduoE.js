import badge from "./badge.png";

function f(attribute) {
    let def = attribute.defend();
    let crit = Math.min(attribute.eCritical, 1);
    let bonus = attribute.eBonus + attribute.bonus + attribute.rockBonus;

    return def * (1 + bonus) * (1 + crit * attribute.criticalDamage);
}

export default {
    name: "abeiduoE",
    chs: "阿贝多-刹那之花",
    description: [
        "使得阿贝多E技能的刹那之花伤害最高",
    ],
    tags: [
        "阿贝多",
    ],
    func: f,
    "for": "abeiduo",
    badge,
}