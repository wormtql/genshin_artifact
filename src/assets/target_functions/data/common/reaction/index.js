import badge from "../badges/water_slime.png";
import config from "./AmpReactionConfig";

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
    chs: "元素反应-增幅反应-期望伤害",
    description: [
        "使得某种元素反应（融化、蒸发）的期望伤害最高"
    ],
    tags: [
        "反应",
    ],
    func: f,
    "for": "common",
    badge,
    needConfig: true,
    needContext: true,
    config,
}