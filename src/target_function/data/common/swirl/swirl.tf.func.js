import reactionBonus from "@/elemental_reaction/reaction_bonus";

function f(attribute) {
    const elementalMastery = attribute.elementalMastery;

    let emBonus = reactionBonus.transformative(elementalMastery);

    let bonus = emBonus + attribute.swirlEnhance;

    return bonus;
}

export default {
    name: "swirl",
    func: f,
    needConfig: false,
}