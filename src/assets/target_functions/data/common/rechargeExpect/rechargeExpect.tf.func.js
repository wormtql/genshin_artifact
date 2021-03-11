function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;
    let re = config.tArgs.recharge;
    let critName = skill === "a" ? "critical" : (skill + "Critical");

    return function(attribute) {
        if (attribute.recharge < re) {
            return attribute.recharge;
        }

        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let crit = Math.min(attribute[critName], 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "rechargeExpect",
    func: f,
    needConfig: true,
}