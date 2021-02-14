export default function (config) {
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

        const attack = attribute.attack();
        const critical = Math.min(attribute.critical, 1);
        const bCritical = Math.min(attribute.bCritical, 1);
        const eCritical = Math.min(attribute.eCritical, 1);
        const qCritical = Math.min(attribute.qCritical, 1);

        let a 
            = (1 - config.aRatio) * (1 + attribute.physicalBonus + attribute.aBonus + attribute.bonus) * config.aTimes         // 物理伤害
            + (config.aRatio) * (1 + attribute[bonusProperty] + attribute.aBonus + attribute.bonus) * config.aTimes            // 元素伤害
        ;
        a = (critical * attribute.criticalDamage + 1) * a;

        let b
            = (1 - config.bRatio) * (1 + attribute.physicalBonus + attribute.bBonus + attribute.bonus) * config.bTimes
            + (config.bRatio) * (1 + attribute[bonusProperty] + attribute.bBonus + attribute.bonus) * config.bTimes
        ;
        b = (bCritical * attribute.criticalDamage + 1) * b;

        let e
            = (1 + attribute[bonusProperty] + attribute.eBonus + attribute.bonus) * config.eTimes
        ;
        e = (eCritical * attribute.criticalDamage + 1) * e;

        let q
            = (1 + attribute[bonusProperty] + attribute.qBonus + attribute.bonus) * config.qTimes
        ;
        q = (qCritical * attribute.criticalDamage + 1) * q;

        let expect = (a * aFreq + b * bFreq + e * eFreq + q * qFreq) * attack;

        return expect;
    }
}