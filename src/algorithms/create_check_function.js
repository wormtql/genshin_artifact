export default function (config) {
    if (config.mode === "any") {
        return function () {
            return true;
        };
    }

    let h;
    if (config.mode === "2") {
        h = {
            [config.setName1]: 2,
        };
    } else if (config.mode === "22") {
        h = {
            [config.setName1]: 2,
            [config.setName2]: 2,
        };
    } else if (config.mode === "4") {
        h = {
            [config.setName1]: 4
        };
    }

    return function (currentSelected) {
        let temp = Object.assign({}, h);
        for (let art of currentSelected) {
            // art might be null, indicating this position is empty
            if (!art) {
                continue;
            }
            if (temp[art.setName] && temp[art.setName] > 0) {
                temp[art.setName]--;
            }
        }
        let sum = Object.values(temp).reduce((a, b) => a + b);
        let left = 5 - currentSelected.length;
        return left >= sum;
    };
}