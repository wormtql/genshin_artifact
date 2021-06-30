const DECAY = 0.7;

export function getShieldValue(shieldConfig, enemyAmount, decay) {
    let ele = shieldConfig.element;

    if (shieldConfig.type === 1) {
        // 无敌盾
        let amount = shieldConfig.amount;

        let value = 0;
        if (decay) {
            value = amount * (1 - Math.pow(DECAY, enemyAmount) / (1 - DECAY));
        } else {
            value = amount * enemyAmount;
        }

        return {
            element: ele,
            value,
        }
    } else if (shieldConfig.type === 2) {
        // 愚人众护甲盾
        let amount = shieldConfig.amount;

        // let value = amount * 
    }
}