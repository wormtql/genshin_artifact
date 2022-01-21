function halfCrit(config) {
    let threshold = config.tArgs.threshold;

    return function (attribute) {
        if (attribute.critical < threshold) {
            return attribute.critical;
        }
    
        return attribute.attack() * (1 + threshold * attribute.criticalDamage);
    };
}

export default {
    name: "aboveCrit",
    func: halfCrit,
    needConfig: true,
}