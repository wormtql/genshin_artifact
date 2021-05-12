function getLength(item) {
    if (typeof item[0] === "object") {
        return item.length
    } else {
        return item[1].length;
    }
}

export default function mergeArray(...items) {
    let ret = [];

    let len = getLength(items[0]);
    for (let i = 0; i < len; i++) {
        let temp = {};

        for (let j = 0; j < items.length; j++) {
            let item = items[j];

            if (typeof item[0] === "object") {
                temp = Object.assign(temp, item[i])
            } else {
                let key = item[0];
                let value = item[1][i];
                temp[key] = value;
            }
        }
        ret.push(temp);
    }

    return ret;
}