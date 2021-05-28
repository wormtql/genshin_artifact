function f(config) {
    const hasTalent1 = config.character.hasTalent1;
    const critExtra = hasTalent1 ? 0.12 : 0;
    
    return function (attribute) {
        let atk = attribute.attack();
        let critical = Math.min(1, attribute.eCritical + critExtra);

        let bonus = attribute.iceBonus + attribute.eBonus + attribute.bonus;
        return atk * (1 + critical * attribute.criticalDamage) * (1 + bonus);
    }
}

export default {
    name: "luoshaliyaE",
    func: f,
    needConfig: true,
}