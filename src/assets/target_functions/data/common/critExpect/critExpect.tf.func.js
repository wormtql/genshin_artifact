function f(config) {
    let threshold = config.tArgs.threshold;
    let skill = config.tArgs.skill;
    let element = config.tArgs.element;
    let critName = skill === "a" ? "critical" : (skill + "Critical");

    return function (attribute) {
        if (attribute[critName] < threshold) {
            return attribute[critName];
        }

        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];
    
        return attribute.attack() * (1 + threshold * attribute.criticalDamage) * (1 + bonus);
    };
}

export default {
    name: "critExpect",
    func: f,
    needConfig: true,
}