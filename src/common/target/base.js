// ab: 普攻占总的频率，例如，普攻三下重击一下，ab = 0.75，如果从不重击，则ab = 1
// dBA: 重击相比普攻的倍率
export function physicalDamage(ab, dBA) {
    return function(attribute) {
        const attack = attribute.attack1 + attribute.attack2;
        const aCritical = Math.max(attribute.critical, 1);
        const bCritical = Math.max(attribute.bCritical, 1);
    
        const aDamage = ab * attack * (aCritical * (1 + attribute.criticalDamage) + (1 - aCritical)) * (1 + attribute.aBonus);
        const bDamage = (1 - ab) * attack * dBA * (bCritical * (1 + attribute.criticalDamage) + (1 - bCritical)) * (1 + attribute.bBonus);
        const damage = (aDamage + bDamage) * (1 + attribute.physicalBonus) * (1 + attribute.bonus);
    
        return damage;
    };
}

export function elementalDamage(element, ab, dBA) {
    // const base = physicalDamage(ab, dBA);

    return function(attribute) {
        const attack = attribute.attack1 + attribute.attack2;
        const aCritical = Math.max(attribute.critical, 1);
        const bCritical = Math.max(attribute.bCritical, 1);
    
        const aDamage = ab * attack * (aCritical * (1 + attribute.criticalDamage) + (1 - aCritical)) * (1 + attribute.aBonus);
        const bDamage = (1 - ab) * attack * dBA * (bCritical * (1 + attribute.criticalDamage) + (1 - bCritical)) * (1 + attribute.bBonus);
        const damage = aDamage + bDamage;

        switch(element) {
            case "fire":
                return damage * (1 + attribute.fireBonus);
            case "ice":
                return damage * (1 + attribute.iceBonus);
            case "rock":
                return damage * (1 + attribute.rockBonus);
            case "thunder":
                return damage * (1 + attribute.thunderBonus);
            case "water":
                return damage * (1 + attribute.waterBonus);
            case "wind":
                return damage * (1 + attribute.windBonus);
        }
    }
}

export function elementalBasic(element, ratio) {
    return function(attribute) {
        const bonusProperty = element + "Bonus";
        const attack = attribute.attack1 + attribute.attack2;
        const critical = Math.max(1.0, attribute.critical);

        let a
            = critical
            * (1 + attribute.criticalDamage)
            * ratio
            * (1 + attribute[bonusProperty] + attribute.bonus)
        ;
        let b
            = critical
            * (1 + attribute.criticalDamage)
            * (1 - ratio)
            * (1 + attribute.physicalBonus + attribute.bonus)
        ;
        let c
            = (1 - critical)
            * ratio
            * (1 + attribute[bonusProperty] + attribute.bonus)
        ;
        let d
            = (1 - critical)
            * (1 - ratio)
            * (1 + attribute.physicalBonus + attribute.bonus)
        ;

        return (a + b + c + d) * attack;
    }
}

export function all(config) {
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


        const bonusProperty = config.element + "Bonus";
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

        return crit + nonCrit;
    }
}