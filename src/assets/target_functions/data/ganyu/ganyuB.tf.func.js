function f(config) {
    let isAmos = config.weapon.name === "amosizhigong";
    let hasTalent1 = config.character.hasTalent1;
    let hasTalent2 = config.character.hasTalent2;

    let ice4Crit = config.tArgs.ice4Crit;
    let talent1Crit = config.tArgs.talent1Crit;
    let talent2Bonus = config.tArgs.talent2Bonus;
    let ele2I = config.tArgs.talent2Bonus;
    let ele2ICrit = config.tArgs.ele2ICrit;

    return function (attribute, context) {
        let attack = attribute.attack();
        let crit = attribute.bCritical;
        if (ele2I) {
            crit += ele2ICrit;
        }

        let bonus = attribute.bBonus + attribute.bonus + attribute.iceBonus;
        if (isAmos) {
            bonus += (config.weapon.refine * 0.02 + 0.06) * 3;
        }

        let isBS4 = (context.artifactSet.blizzardStrayer || 0) >= 4;
        if (isBS4) {
            crit += ice4Crit;
        }
        if (hasTalent1) {
            crit += talent1Crit;
        }
        if (hasTalent2) {
            bonus += talent2Bonus;
        }

        return attack * (1 + bonus) * (1 + Math.min(crit, 1) * attribute.criticalDamage);
    };
}

export default {
    name: "ganyuB",
    func: f,
    needConfig: true,
    needContext: true,
}