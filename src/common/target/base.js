function d_all(config, attribute) {
    const bonusProperty = config.element + "Bonus";

    let fBonus = attribute[bonusProperty];
    let attack = attribute.attack1 + attribute.attack2;
    let attack1 = attribute.attack1;

    let aRatio = config.aRatio;
    let aTimes = config.aTimes;
    let aFreq = config.aFreq;

    let bRatio = config.bRatio;
    let bTimes = config.bTimes;
    let bFreq = config.bFreq;

    let eTimes = config.eTimes;
    let eFreq = config.eFreq;

    let qTimes = config.qTimes;
    let qFreq = config.qFreq;

    let aBonus = attribute.aBonus;
    let bBonus = attribute.bBonus;
    let eBonus = attribute.eBonus;
    let qBonus = attribute.qBonus;
    let physicalBonus = attribute.physicalBonus;
    let bonus = attribute.bonus;

    let recharge = attribute.recharge;

    let critical = attribute.critical;
    let criticalDamage = attribute.criticalDamage;

    let d_critical = critical >= 1 ? 0
        : (attack*criticalDamage*(aFreq*aTimes*(aRatio*(aBonus + bonus + fBonus + 1) - (aRatio - 1)*(aBonus + bonus + physicalBonus + 1)) + bFreq*bTimes*(bRatio*(bBonus + bonus + fBonus + 1) - (bRatio - 1)*(bBonus + bonus + physicalBonus + 1)) + eFreq*eTimes*(bonus + eBonus + fBonus + 1) + qFreq*qTimes*recharge*(bonus + fBonus + qBonus + 1))/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100;

    return [
        {
            name: "critical",
            chs: "暴击率",
            d: d_critical,
        },
        {
            name: "criticalDamage",
            chs: "暴击伤害",
            d: (attack*critical*(aFreq*aTimes*(aRatio*(aBonus + bonus + fBonus + 1) - (aRatio - 1)*(aBonus + bonus + physicalBonus + 1)) + bFreq*bTimes*(bRatio*(bBonus + bonus + fBonus + 1) - (bRatio - 1)*(bBonus + bonus + physicalBonus + 1)) + eFreq*eTimes*(bonus + eBonus + fBonus + 1) + qFreq*qTimes*recharge*(bonus + fBonus + qBonus + 1))/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "fixedAttack",
            chs: "固定攻击力",
            d: (critical*(criticalDamage + 1) - critical + 1)*(aFreq*aTimes*(aRatio*(aBonus + bonus + fBonus + 1) - (aRatio - 1)*(aBonus + bonus + physicalBonus + 1)) + bFreq*bTimes*(bRatio*(bBonus + bonus + fBonus + 1) - (bRatio - 1)*(bBonus + bonus + physicalBonus + 1)) + eFreq*eTimes*(bonus + eBonus + fBonus + 1) + qFreq*qTimes*recharge*(bonus + fBonus + qBonus + 1))/(aFreq + bFreq + eFreq + qFreq*recharge)
        },
        {
            name: "pAttack",
            chs: "%攻击力",
            d: (attack1*(critical*(criticalDamage + 1) - critical + 1)*(aFreq*aTimes*(aRatio*(aBonus + bonus + fBonus + 1) - (aRatio - 1)*(aBonus + bonus + physicalBonus + 1)) + bFreq*bTimes*(bRatio*(bBonus + bonus + fBonus + 1) - (bRatio - 1)*(bBonus + bonus + physicalBonus + 1)) + eFreq*eTimes*(bonus + eBonus + fBonus + 1) + qFreq*qTimes*recharge*(bonus + fBonus + qBonus + 1))/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "physicalBonus",
            chs: "物伤加成",
            d: (attack*(aFreq*aTimes*(aRatio - 1) + bFreq*bTimes*(bRatio - 1))*(-critical*(criticalDamage + 1) + critical - 1)/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "fBonus",
            chs: "元素伤害加成",
            d: (attack*(critical*(criticalDamage + 1) - critical + 1)*(aFreq*aRatio*aTimes + bFreq*bRatio*bTimes + eFreq*eTimes + qFreq*qTimes*recharge)/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "aBonus",
            chs: "普攻伤害加成",
            d: (aFreq*aTimes*attack*(critical*criticalDamage + 1)/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "bBonus",
            chs: "重击伤害加成",
            d: (attack*bFreq*bTimes*(critical*criticalDamage + 1)/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "eBonus",
            chs: "元素战技伤害加成",
            d: (attack*bFreq*bTimes*(critical*criticalDamage + 1)/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "qBonus",
            chs: "元素爆发伤害加成",
            d: (attack*qFreq*qTimes*recharge*(critical*criticalDamage + 1)/(aFreq + bFreq + eFreq + qFreq*recharge)) / 100,
        },
        {
            name: "recharge",
            chs: "充能效率",
            d: (attack*qFreq*(qTimes*(critical*(criticalDamage + 1) - critical + 1)*(aFreq + bFreq + eFreq + qFreq*recharge)*(bonus + fBonus + qBonus + 1) + (-critical*(criticalDamage + 1) + critical - 1)*(aFreq*aTimes*(aRatio*(aBonus + bonus + fBonus + 1) - (aRatio - 1)*(aBonus + bonus + physicalBonus + 1)) + bFreq*bTimes*(bRatio*(bBonus + bonus + fBonus + 1) - (bRatio - 1)*(bBonus + bonus + physicalBonus + 1)) + eFreq*eTimes*(bonus + eBonus + fBonus + 1) + qFreq*qTimes*recharge*(bonus + fBonus + qBonus + 1)))/(aFreq + bFreq + eFreq + qFreq*recharge)**2) / 100
        }
    ]
}

export function all(config) {
    const bonusProperty = config.element + "Bonus";

    return function(attribute) {
        let qFreq = config.qFreq * attribute.recharge;
        const sum = qFreq + config.aFreq + config.bFreq + config.eFreq;
        if (sum === 0) {
            return -Infinity;
        }

        let aFreq = config.aFreq / sum;
        let bFreq = config.bFreq / sum;
        let eFreq = config.eFreq / sum;
        qFreq = qFreq / sum;

        const attack = attribute.attack1 + attribute.attack2;
        const critical = Math.max(attribute.critical, 1);

        let a 
            = (1 - config.aRatio) * (1 + attribute.physicalBonus + attribute.aBonus + attribute.bonus) * config.aTimes         // 物理伤害
            + (config.aRatio) * (1 + attribute[bonusProperty] + attribute.aBonus + attribute.bonus) * config.aTimes            // 元素伤害
        ;
        let b
            = (1 - config.bRatio) * (1 + attribute.physicalBonus + attribute.bBonus + attribute.bonus) * config.bTimes
            + (config.bRatio) * (1 + attribute[bonusProperty] + attribute.bBonus + attribute.bonus) * config.bTimes
        ;
        let e
            = (1 + attribute[bonusProperty] + attribute.eBonus + attribute.bonus) * config.eTimes
        ;
        let q
            = (1 + attribute[bonusProperty] + attribute.qBonus + attribute.bonus) * config.qTimes
        ;

        let expect = a * aFreq + b * bFreq + e * eFreq + q * qFreq;
        let crit = critical * (1 + attribute.criticalDamage) * expect * attack;
        let nonCrit = (1 - critical) * expect * attack;

        let ret = {
            "deritives": d_all(config, attribute),
            "value": crit + nonCrit
        }

        return ret;
    }
}