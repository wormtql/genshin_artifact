import reaction from "@/elemental_reaction/reaction_bonus";
import { getTransformativeBase, REACTION_TYPE } from "@/elemental_reaction/transformativeBase";
import { charactersData } from "@asset/characters";


let skill = charactersData["wendi"].skill;

function f(config) {
    let sr = Math.min(config.tArgs.swirlRatio, 1);
    let normal = 1 - sr;

    let swirlTime = config.tArgs.swirlTime;
    let rate1 = config.tArgs.rate1;
    let rate2 = config.tArgs.rate2;
    let isGroup = config.tArgs.isGroup;

    let level = config.cArgs.level;
    let qRatio = skill.q.dmg1[config.cArgs.skill3 - 1];
    let qRatio2 = skill.q.dmg2[config.cArgs.skill3 - 1];

    return function (attribute, context) {
        let isVV4 = context.artifactSet.viridescentVenerer >= 4;

        let atk = attribute.attack();
        let crit = Math.min(1, attribute.qCritical);
        let bonus = attribute.bonus + attribute.windBonus + attribute.qBonus;

        let emBonus = reaction.transformative(attribute.elementalMastery);
        let transformativeBonus = emBonus + attribute.swirlEnhance;

        let normalDamage = atk * (1 + crit * attribute.criticalDamage) * (1 + bonus) * qRatio;
        let eleDamage = atk * (1 + crit * attribute.criticalDamage) * (1 + attribute.bonus + attribute.qBonus) * qRatio2;
        let swirlDamage = getTransformativeBase(level, REACTION_TYPE.SWIRL) * (1 + transformativeBonus);

        if (isVV4) {
            return normalDamage * normal * 0.9
                + eleDamage * (1 - swirlTime) * (rate1 * 1.15 + (1 - rate1) * 0.9)
                + swirlDamage * sr * (rate2 * 1.15 + (1 - rate2) * 0.9) * (isGroup ? 2 : 1);
        } else {
            return normalDamage * normal * 0.9
                + eleDamage * (1 - swirlTime) * 0.9
                + swirlDamage * sr * 0.9 * (isGroup ? 2 : 1);
        }
    }
}

export default {
    name: "ventiSwirl",
    func: f,
    needConfig: true,
    needContext: true,
}