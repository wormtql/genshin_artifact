import { charactersData } from "@asset/characters";
import { TargetFuncUtils } from "../../../utils";


const skill = charactersData["abeiduo"].skill;

function f(attribute, { cArgs, tArgs }) {
    const isConste2 = cArgs.constellation >= 2;
    const skill2 = cArgs.skill2;
    const skill3 = cArgs.skill3;

    const eRatio1 = skill.e.dmg1[skill2 - 1];
    const eRatio2 = skill.e.dmg2[skill2 - 1];
    const qRatio1 = skill.q.dmg1[skill3 - 1];
    const qRatio2 = skill.q.dmg2[skill3 - 1];

    const eCount = tArgs.eCount;
    const qCount = tArgs.qCount;
    const qFreq = tArgs.qFreq;
    const c2Count = tArgs.c2Count;

    const def = attribute.defend();
    const atk = attribute.attack();

    const eDamage1 = TargetFuncUtils.damageDefault(attribute, eRatio1 * atk, "rock", "e").expect;
    const eDamage2 = TargetFuncUtils.damageDefault(attribute, eRatio2 * def, "rock", "e").expect * eCount;
    const eDamage = eDamage1 + eDamage2;

    const qBaseBonus = isConste2 ? (def * 0.3 * c2Count) : 0;
    const qDamage1 = TargetFuncUtils.damageDefault(attribute, qRatio1 * atk + qBaseBonus, "rock", "q").expect;
    const qDamage2 = TargetFuncUtils.damageDefault(attribute, qRatio2 * atk + qBaseBonus, "rock", "q").expect * qCount;
    const qDamage = qDamage1 + qDamage2;

    return eDamage * (1 - qFreq) + qDamage * qFreq;
}

export default {
    name: "abeiduoDefault",
    func: f,
    needConfig: true,
    needContext: true,
    version: 2,
}