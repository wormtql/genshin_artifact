let cache = {};

function getBaseValue(tagName) {
    let temp;
    switch (tagName) {
        case "attackPercentage":
            temp = 0.058;
            break;
        case "lifeStatic":
            temp = 299;
            break;
        case "attackStatic":
            temp = 19;
            break;
        case "defendStatic":
            temp = 23;
            break;
        case "elementalMastery":
            temp = 23;
            break;
        case "recharge":
            temp = 0.065;
            break;
        case "defendPercentage":
            temp = 0.073;
            break;
        case "lifePercentage":
            temp = 0.058;
            break;
        case "critical":
            temp = 0.039;
            break;
        case "criticalDamage":
            temp = 0.078;
            break;
    }

    cache[tagName] = temp;
    return temp;
}

function f(args) {
    let weight = args.pArgs.weight;

    return function (tags) {
        let value = 0;
        for (let tag of tags) {
            if (!Object.prototype.hasOwnProperty.call(weight, tag.name)) {
                continue;
            }
            if (weight[tag.name][0]) {
                let w = weight[tag.name][1];
                let baseValue = cache[tag.name] ?? getBaseValue(tag.name);
                value += w * tag.value / baseValue;
            }
        }

        return value;
    };
}

function validFunc(args) {
    let validTags = [];
    let weight = args.pArgs.weight;
    for (let key in weight) {
        if (weight[key][0]) {
            validTags.push(key);
        }
    }

    return validTags;
}

export default {
    name: "ratio",
    func: f,
    valid: validFunc,
    needConfig: true,
}