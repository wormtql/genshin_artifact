export function computeShieldRate(enemies, characters) {
    let enemiesCount = enemies.length;
    let alpha = [0, 0, 0, 0, 0, 0, 0];
    let shieldCount = 0;

    for (let i = 0; i < enemiesCount; i++) {
        if (enemies[i].getProperty("abyss.hasShield")) {
            shieldCount++;
        }
    }

    if (shieldCount === 0) {
        return 1;
    }

    for (let i = 0; i < 7; i++) {
        let acc = 1;
        for (let j = 0; j < enemiesCount; j++) {
            if (enemies[j].getProperty("abyss.hasShield")) {
                acc *= enemies[j].getProperty("abyss.shieldRate", i);
            }
        }
        // console.log(acc);
        // alpha[i] = Math.pow(acc, 1 / shieldCount);
        alpha[i] = acc;
    }
    console.log("alpha", alpha);

    let ans = 0;
    for (let i = 0, l = characters.length; i < l; i++) {
        let temp = 0;
        for (let j = 0; j < 7; j++) {
            let temp2 = characters[i].abyss.shieldRate[j] * alpha[j];
            temp = Math.max(temp, temp2);
        }
        ans += temp;
    }

    return ans;
}

export function computeDamageRate(enemies, characters) {
    let t = [];

    let bonus = [0, 0, 0, 0, 0, 0, 0];
    for (let i = 0; i < 7; i++) {
        for (let j = 0; j < characters.length; j++) {
            bonus[i] += characters[j].abyss.bonusRate[i];
        }
        bonus[i] += 1;
    }
    

    for (let i = 0; i < enemies.length; i++) {
        let temp = 0;
        for (let j = 0; j < characters.length; j++) {
            let acc = 0;
            for (let k = 0; k < 7; k++) {
                acc += enemies[i].getProperty("abyss.damagedRate", k) * characters[j].abyss.damageRate[k] * bonus[k];
            }
            temp += acc;
        }
        t.push(enemies[i].getProperty("abyss.hp") / temp);
    }

    let characterJuguai = 0;
    for (let i = 0; i < characters.length; i++) {
        characterJuguai = Math.max(characterJuguai, characters[i].abyss.juguaiRate ?? 0);
    }

    let s = [];
    for (let i = 0; i < enemies.length; i++) {
        s.push(1 - (1 - enemies[i].getProperty("abyss.weight")) * characterJuguai);
    }

    let L = 0;
    for (let i = 0; i < enemies.length; i++) {
        L += 1 - s[i];
    }

    let T = 0;
    for (let i = 0; i < enemies.length; i++) {
        if (L > 1e-6) {
            T += (1 + (L - 1) * s[i]) / L * t[i];
        } else {
            T += t[i];
        }
    }

    return T;
}