function f(args) {
    let baseAtk = args.pArgs.baseAtk;
    let baseCrit = args.pArgs.baseCrit;
    let baseCD = args.pArgs.baseCriticalDamage;
    let atk = args.pArgs.atk;

    let original = atk * (1 + baseCrit * baseCD);

    return function (tags) {
        let a = atk;
        let crit = baseCrit;
        let cd = baseCD;
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
            }
        }

        let value = a * (1 + cd * crit);
        // return (value - original) / value * 100;
        return value - original;
    };
}

export default {
    name: "damage",
    func: f,
    valid: ["attackStatic", "attackPercentage", "critical", "criticalDamage"],
    needConfig: true,
}