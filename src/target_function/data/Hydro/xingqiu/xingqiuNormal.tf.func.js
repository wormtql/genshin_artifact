import { charactersData } from "@asset/characters";


const skill = charactersData["xingqiu"].skill;

// let flag = false;

function xingqiuNormal(attribute, { cArgs, wArgs }) {
    const isConste2 = cArgs.constellation >= 2;
    const isConste4 = cArgs.constellation >= 4;
    const isConste6 = cArgs.constellation >= 6;

    const skill2 = cArgs.skill2;
    const skill3 = cArgs.skill3;

    const eRatio = skill.e.dmg1[skill2 - 1] + skill.e.dmg2[skill2 - 1];
    const qRatio = skill.q.dmg1[skill3 - 1] * (isConste4 ? 1.5 : 1);

    const isJili = wArgs.name === "jilijian";
    // 每秒几次E技能
    const eFreq = isJili ? 0.09 : 0.045;

    // 每秒由E产生的微粒
    const eParticle = (isJili ? 10 : 5) * eFreq * 3;
    // 每秒环境产生的微粒
    const envParticle = 0.8;
    const qExtraTime = 3;
    // 充能完成的需要的时间
    const energyEff = Math.max(80 / ((eParticle + envParticle) * (1 + attribute.recharge)), 20) + qExtraTime;
    // if (!flag) {
    //     console.log(energyEff);
    //     flag = true;
    // }

    // q平均每次剑数
    const qSwordCountPerRound = isConste6 ? 3.33 : 2.5;
    // 每秒打出的剑数
    const qSwordFreq = isConste2 ? (15 * qSwordCountPerRound) / energyEff : (12 * qSwordCountPerRound) / energyEff;


    const eBonus = attribute.waterBonus + attribute.eBonus + attribute.bonus;
    const qBonus = attribute.waterBonus + attribute.qBonus + attribute.bonus;
    const atk = attribute.attack();
    const eCR = Math.min(1, attribute.eCritical);
    const qCR = Math.min(1, attribute.qCritical);
    const criticalDamage = attribute.criticalDamage;

    const eDps = eRatio * eFreq * (1 + eBonus) * (1 + eCR * criticalDamage);
    const qDps = qSwordFreq * qRatio * (1 + qBonus) * (1 + qCR * criticalDamage)
    const dps = (eDps + qDps) * atk;

    return dps;
}

export default {
    name: "xingqiuNormal",
    func: xingqiuNormal,
    needConfig: false,
    needContext: false,
    version: 2,
}