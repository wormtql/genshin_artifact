import elementalBonus from "@/elemental_reaction/reaction_bonus";

function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;
    let reactionType = config.tArgs.reactionType;
    let critName = skill === "a" ? "critical" : (skill + "Critical");

    return function(attribute) {
        let atk = attribute.attack();
        let reactBonus = elementalBonus.amp(attribute.elementalMastery);
        if (reactionType === "vaporize") {
            reactBonus += attribute.vaporizeEnhance;
        } else if (reactionType === "melt") {
            reactBonus += attribute.meltEnhance;
        }

        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let crit = Math.min(attribute[critName], 1);
        crit = Math.max(0, crit);

        return atk * (1 + bonus) * (crit * attribute.criticalDamage + 1) * (1 + reactBonus);
    };
}

export default {
    name: "reactionAmp",
    func: f,
    needConfig: true,
}