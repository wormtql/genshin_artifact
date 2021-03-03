import badge from "./badge.png";
import config from "./HutaoConfig";
// import result from "./HutaoResult";

import skill from "./skill";

function f(config) {
    let baseAtk = config.character.baseAtk + config.weapon.baseAtk;
    let eLevel = config.cArgs.skill2;
    let aLevel = config.cArgs.skill1;
    let atkInc = skill.e.hp[eLevel - 1];

    let aT = skill.a.dmg3[aLevel - 1];
    let bT = skill.b.dmg[aLevel - 1];

    let hasTalent2 = config.character.hasTalent2;

    let hpBelow50 = config.tArgs.hpBelow50;
    let pyroRate = config.tArgs.pyroRate;
    let evaporate = config.tArgs.evaporate;
    let melt = config.tArgs.melt;
    let normal = 1 - evaporate - melt;
    let bFreq = config.tArgs.bFreq;
    let aFreq = 1 - bFreq;
    //console.log(aFreq, bFreq);
    //console.log(aT, bT);
    // console.log(evaporate, melt, normal);

    let talentBonus = (hasTalent2 && hpBelow50) ? 0.33 : 0;

    return function (attribute, context) {
        let isCW4 = (context.artifactSet.crimsonWitch || 0) >= 4;
        let isLW4 = (context.artifactSet.lavaWalker || 0) >= 4;

        let atkBonus = attribute.life() * atkInc;
        atkBonus = Math.min(atkBonus, 4 * baseAtk);

        let atk = attribute.attack() + atkBonus;
        let commonBonus = attribute.bonus + attribute.fireBonus + talentBonus;

        let em = attribute.elementalMastery;
        let amp = 20 * em / (3 * (em + 1400));
        if (isCW4) {
            commonBonus += 0.075;
            amp += 0.15;
        }
        if (isLW4) {
            commonBonus += pyroRate * 0.35;
        }

        let crit = Math.min(1, attribute.critical);

        let a = atk * (1 + commonBonus + attribute.aBonus) * aFreq * aT;
        let b = atk * (1 + commonBonus + attribute.bBonus) * bFreq * bT;

        return (a + b) * (1 + crit * attribute.criticalDamage) * (normal + (evaporate * 1.5 + melt * 2) * (1 + amp));

        // return atk * (1 + bonus) * (1 + crit * attribute.criticalDamage) * (normal + (evaporate * 1.5 + melt * 2) * (1 + amp))
        //     * (aFreq * aT + bFreq * bT)
        // ;
    };
}

export default {
    name: "hutaoE",
    chs: "胡桃-蝶引来生",
    description: [
        "使得胡桃E技能后的攻击伤害最高",
    ],
    tags: [
        "输出",
        "胡桃",
    ],
    func: f,
    "for": "hutao",
    badge,
    needConfig: true,
    needContext: true,
    config,
    // result,
}