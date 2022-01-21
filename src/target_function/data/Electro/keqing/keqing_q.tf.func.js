function f(attribute) {
    let atk = attribute.attack();
    let crit = Math.min(1, attribute.qCritical + 0.15);

    let bonus = attribute.qBonus + attribute.thunderBonus + attribute.bonus;

    return atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
}


export default {
    name: "keqingQ",
    func: f,
    // needConfig: true,
    // needContext: true,
}