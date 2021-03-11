function xiaoQ(attribute) {
    let attack = attribute.attack();
    let crit = Math.min(attribute.airCritical, 1);

    let bonus = attribute.airBonus + attribute.bonus + attribute.windBonus;
    let baseDmg = attack * (1 + bonus);

    return (crit * attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "xiaoQ",
    func: xiaoQ,
}