import { charactersData } from "@asset/characters";


const skill = charactersData["dadaliya"].skill;

function f(attribute, { cArgs }) {
    // const isConste2 = cArgs.constellation >= 2;
    // const isConste4 = cArgs.constellation >= 4;
    // const isConste6 = cArgs.constellation >= 6;

    const skill1 = cArgs.skill1;
    const skill2 = cArgs.skill2;
    const skill3 = cArgs.skill3;

    const aBonus = attribute.bonus + attribute.waterBonus + attribute.aBonus;
    const bBonus = attribute.bonus + attribute.waterBonus + attribute.bBonus;
    const eBonus = attribute.bonus + attribute.waterBonus + attribute.eBonus;
    const qBonus = attribute.bonus + attribute.waterBonus + attribute.qBonus;
    const atk = attribute.attack();
    const criticalDamage = attribute.criticalDamage;
    const aCritRate = attribute.critical;
    const bCritRate = attribute.bCritical;
    const eCritRate = attribute.eCritical;
    const qCritRate = attribute.qCritical;

    const aRatio = skill.e.dmg1[skill2 - 1] + skill.e.dmg2[skill2 - 1] + skill.e.dmg3[skill2 - 1] + skill.e.dmg4[skill2 - 1];
    const bRatio = skill.e.bDmg11[skill2 - 1] + skill.e.bDmg12[skill2 - 1];
    const qRatio = skill.q.dmg1[skill3 - 1];

    const duanliuShanRatio = skill.a.dmg7[skill1 - 1] * 3;
    const duanliuPoRatio = skill.a.dmg8[skill1 - 1];
    const duanliuZhanRatio = skill.e.eDmg2[skill2 - 1];
    const duanliuBaoRatio = skill.q.dmg3[skill3 - 1];

    const aDmg = aRatio * (1 + aCritRate * criticalDamage) * (1 + aBonus) * atk;
    const bDmg = bRatio * (1 + bCritRate * criticalDamage) * (1 + bBonus) * atk;
    const qDmg = qRatio * (1 + qCritRate * criticalDamage) * (1 + qBonus) * atk;
    const duanliuShanDmg = duanliuShanRatio * (1 + aCritRate * criticalDamage) * (1 + aBonus) * atk;
    const duanliuPoDmg = duanliuPoRatio * (1 + aCritRate * criticalDamage) * (1 + aBonus) * atk;
    const duanliuZhanDmg = duanliuZhanRatio * (1 + eCritRate * criticalDamage) * (1 + eBonus) * atk;
    const duanliuBaoDmg = duanliuBaoRatio * (1 + qCritRate * criticalDamage) * (1 + qBonus) * atk;

    const dmg
        = aDmg
        + bDmg
        + qDmg
        + duanliuShanDmg * 0.1
        + duanliuPoDmg * 0.5
        + duanliuZhanDmg * 1
        + duanliuBaoDmg * 0.5
    ;

    return dmg;
}

export default {
    name: "dadaliyaNormal",
    func: f,
    needConfig: false,
    needContext: false,
    version: 2,
}