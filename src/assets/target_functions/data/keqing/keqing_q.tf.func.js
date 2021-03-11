function f(config) {
    let isXiali = config.weapon.name === "xialilongyin";
    let refine = config.weapon.refine;

    return function (attribute) {
        let atk = attribute.attack();
        let crit = Math.min(1, attribute.qCritical + 0.15);

        let bonus = attribute.qBonus + attribute.thunderBonus + attribute.bonus;
        if (isXiali) {
            bonus += 0.04 * refine + 0.16;
        }
    
        return atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    }
}


export default {
    name: "keqingQ",
    func: f,
    needConfig: true,
    needContext: true,
}