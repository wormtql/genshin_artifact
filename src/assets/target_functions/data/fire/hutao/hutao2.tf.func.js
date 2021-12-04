import { charactersData } from "@asset/characters";
import { TargetFuncUtils } from "../../../utils";

const skill = charactersData["hutao"].skill;


function hutao2(attribute, { tArgs, cArgs }) {
    const baseAtk = attribute.attackBasic;
    const atkBonusDefRatio = skill.e.hp[cArgs.skill2 - 1];
    const atkBonus = Math.min(atkBonusDefRatio * attribute.life(), 4 * baseAtk);
    const atk = attribute.attack() + atkBonus;

    const vaporizeRate = Math.max(Math.min(tArgs.vaporize, 1), 0);
    const normalRate = 1 - vaporizeRate;
    const bRate = tArgs.bFreq;
    const aRate = 1 - bRate;

    const aRatio = (skill.a.dmg1[cArgs.skill1 - 1] + skill.a.dmg2[cArgs.skill1 - 1] + skill.a.dmg3[cArgs.skill1 - 1]) / 3;
    const bRatio = skill.a.bDmg[cArgs.skill1 - 1];

    const aDamage = TargetFuncUtils.damageDefault(attribute, aRatio * atk, "fire", "a").expect;
    const bDamage = TargetFuncUtils.damageDefault(attribute, bRatio * atk, "fire", "b").expect;
    const aVaporizeDamage = TargetFuncUtils.damageReactionDefault(attribute, aRatio * atk, "fire", "a", "vaporize").expect
    const bVaporizeDamage = TargetFuncUtils.damageReactionDefault(attribute, bRatio * atk, "fire", "b", "vaporize").expect

    const dps
        = aDamage * aRate * normalRate
        + bDamage * bRate * normalRate
        + aVaporizeDamage * aRate * vaporizeRate
        + bVaporizeDamage * bRate * vaporizeRate

    return dps
}

export default {
    name: "hutao2",
    func: hutao2,
    needConfig: false,
    needContext: false,
    version: 2
}