import positions from "@const/positions";

export function count(artifacts) {
    let c = 0;

    positions.forEach(pos => {
        c += artifacts[pos].length;
    });

    return c;
}

export function howManyUpgradeCount(value, slots) {
    for (let i = 0; i < 4; i++) {
        if (Math.abs(slots[i] - value) < 0.0001) {
            return 1;
        }
    }

    let max = Math.floor(value / slots[0]);
    let min = Math.ceil(value / slots[3]);

    if (max === min) {
        return max
    }

    let helper = (value, ttl) => {
        if (value < 0) {
            return false;
        }
        if (ttl === 0) {
            return Math.abs(value) < 0.01;
        }
        
        for (let i = 0; i < 4; i++) {
            let newValue = value - slots[i];
            let temp = helper(newValue, ttl - 1);
            if (temp) {
                return true;
            }
        }

        return false;
    };
    console.log(value, slots);

    for (let i = min; i <= max; i++) {
        let temp = helper(value, i);
        if (temp) {
            return true;
        }
    }

    return (min + max) / 2;
} 