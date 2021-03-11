import elementalBonus from "@/elemental_reaction/reaction_bonus";

function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;
    let critName = skill === "a" ? "critical" : (skill + "Critical");

    return function(attribute, context) {
        let isCM4 = (context.artifactSet.crimsonWitch || 0) >= 4;
        let atk = attribute.attack();
        let reactBonus = elementalBonus.amp(attribute.elementalMastery);
        if (isCM4) {
            reactBonus += 0.15;
        }

        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let crit = Math.min(attribute[critName], 1);

        return atk * (1 + bonus) * (crit * attribute.criticalDamage + 1) * (1 + reactBonus);
    };
}

export default {
    name: "reactionAmp",
    func: f,
    needConfig: true,
    needContext: true,
}