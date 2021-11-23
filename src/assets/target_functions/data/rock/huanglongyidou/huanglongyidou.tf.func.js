import { charactersData } from "@asset/characters";


const skill = charactersData["huanglongyidou"].skill;

function f(attribute, { cArgs }) {
    const hasTalent2 = cArgs.level > 60 || (cArgs.level === 60 && cArgs.ascend);

    const def = attribute.defend();
    const atkBonus = skill.q.atkBonus[cArgs.skill3 - 1] * def;
    const atk = attribute.attack() + atkBonus;
    const hp = attribute.life();

    const aAtkRatio = attribute.atkRatio + attribute.aAtkRatio + skill.a.dmg1[cArgs.skill1 - 1];
    const b1AtkRatio = attribute.atkRatio + attribute.bAtkRatio + skill.a.bDmg3[cArgs.skill1 - 1];
    const b2AtkRatio = attribute.atkRatio + attribute.bAtkRatio
        + (skill.a.bDmg1[cArgs.skill1 - 1] * 0.7 + skill.a.bDmg2[cArgs.skill1 - 1] * 0.3);
    const aDefRatio = attribute.defendRatio + attribute.aDefRatio;
    const bDefRatio = attribute.defendRatio + attribute.bDefRatio;
    const aHpRatio = attribute.lifeRatio + attribute.aLifeRatio;
    const bHpRatio = attribute.lifeRatio + attribute.bLifeRatio;

    const aBaseDmg = atk * aAtkRatio + def * aDefRatio + hp * aHpRatio;
    const b1BaseDmg = atk * b1AtkRatio + def * bDefRatio + hp * bHpRatio;
    const b2BaseDmg = atk * b2AtkRatio + def * (bDefRatio + (hasTalent2 ? 0.35 : 0)) + hp * bHpRatio;

    const aBonus = attribute.bonus + attribute.rockBonus + attribute.aBonus;
    const bBonus = attribute.bonus + attribute.rockBonus + attribute.bBonus;

    const aCR = Math.min(1, attribute.critical);
    const bCR = Math.min(1, attribute.bCritical);
    const cd = attribute.criticalDamage;

    const dps
        = 0.5 * aBaseDmg * (1 + aBonus) * (1 + aCR * cd)
        + 0.85 * b1BaseDmg * (1 + bBonus) * (1 + bCR * cd)
        + 0.15 * b2BaseDmg * (1 + bBonus) * (1 + bCR * cd)

    return dps;
}

export default {
    name: "huanglongyidouNormal",
    func: f,
    needConfig: false,
    needContext: false,
    version: 2,
}