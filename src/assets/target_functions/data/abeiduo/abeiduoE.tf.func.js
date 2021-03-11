function f(attribute) {
    let def = attribute.defend();
    let crit = Math.min(attribute.eCritical, 1);
    let bonus = attribute.eBonus + attribute.bonus + attribute.rockBonus;

    return def * (1 + bonus) * (1 + crit * attribute.criticalDamage);
}

export default {
    name: "abeiduoE",
    func: f,
}