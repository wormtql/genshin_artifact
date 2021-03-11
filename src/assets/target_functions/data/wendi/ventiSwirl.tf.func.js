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
    func: f,
    needConfig: true,
    needContext: true,
}