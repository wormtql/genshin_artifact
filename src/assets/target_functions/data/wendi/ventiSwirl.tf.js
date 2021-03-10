import badge from "@asset/badges/wendi.png";
import config from "./VentiSwirlConfig";

import reaction from "@/elemental_reaction/reaction_bonus";
import reactionConst from "@/elemental_reaction/reaction_constants";

let jubian = reaction.jubian;
let s = reactionConst.swirl;

function f(config) {
    let sr = Math.min(config.tArgs.swirlRatio, 1);
    let normal = 1 - sr;

    return function (attribute, context) {
        let isVV4 = (context.artifactSet.viridescentVenerer || 0) >= 4;

        let atk = attribute.attack();
        let crit = Math.min(1, attribute.qCritical);
        let bonus = attribute.bonus + attribute.windBonus + attribute.qBonus;

        let amp = jubian(attribute.elementalMastery);
        if (isVV4) {
            amp += 0.6;
        }

        return atk * (1 + crit * attribute.criticalDamage) * (1 + bonus) * (normal + s * sr * (1 + amp));
    }
}

export default {
    name: "ventiSwirl",
    chs: "温迪-风神之诗",
    description: [
        "使得Q的期望伤害最高（考虑扩散）",
    ],
    tags: [
        "反应",
        "温迪",
    ],
    func: f,
    "for": "wendi",
    badge,
    needConfig: true,
    needContext: true,
    config,
}