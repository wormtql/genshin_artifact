function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let baseDmg = atk * (1 + bonus);
        return (attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "max",
    func: f,
    needConfig: true,
}