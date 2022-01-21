function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;
    let critName = skill === "a" ? "critical" : (skill + "Critical");

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let crit = Math.min(attribute[critName], 1);
        crit = Math.max(0, crit);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "expect",
    func: f,
    needConfig: true,
}