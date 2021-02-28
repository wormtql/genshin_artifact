import badge from "./badges/fire_slime.png";


function f(attribute, context) {
    let atk = attribute.attack();
    let em = attribute.elementalMastery;
    let amp = 20 * em / (3 * (em + 1400));
    let bonus = attribute.bonus + attribute.fireBonus;
    let crit = Math.min(attribute.critical, 1);

    let isCW4 = (context.artifactSet.crimsonWitch || 0) >= 4;

    let baseDmg = atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    return baseDmg * (1 + amp + (isCW4 ? 0.15 : 0));
}

export default {
    name: "eleMeltPyro",
    chs: "元素反应-融化（火->冰）",
    description: [
        "使得融化（火->冰）期望伤害最高"
    ],
    tags: [
        "元素反应",
    ],
    func: f,
    "for": "common",
    badge,
    needContext: true,
}