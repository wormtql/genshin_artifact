function f(args) {
    let baseAtk = args.pArgs.baseAtk;
    let baseCrit = args.pArgs.baseCrit;
    let baseCD = args.pArgs.baseCriticalDamage;
    let atk = args.pArgs.atk;
    let baseHP = args.pArgs.baseHP;
    let hp = args.pArgs.hp;

    let original = (atk + Math.min(4 * baseAtk, 0.0536 * hp)) * (1 + baseCrit * baseCD);

    return function (tags) {
        let a = atk;
        let crit = baseCrit;
        let cd = baseCD;
        let myHp = hp;
        for (let tag of tags) {
            switch (tag.name) {
                case "attackStatic":
                    a += tag.value;
                    break;
                case "attackPercentage":
                    a += tag.value * baseAtk;
                    break;
                case "critical":
                    crit += tag.value;
                    break;
                case "criticalDamage":
                    cd += tag.value;
                    break;
                case "lifeStatic":
                    myHp += tag.value;
                    break;
                case "lifePercentage":
                    myHp += tag.value * baseHP;
                    break;
            }
        }

        let value = (a + Math.min(4 * baseAtk, 0.0536 * myHp)) * (1 + cd * crit);
        // return (value - original) / value * 100;
        return value - original;
    };
}

export default {
    name: "hutaoE",
    func: f,
    valid: ["attackStatic", "attackPercentage", "critical", "criticalDamage", "lifeStatic", "lifePercentage"],
    needConfig: true,
}