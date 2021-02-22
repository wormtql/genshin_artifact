function deepCopy(x) {
    if (Array.isArray(x)) {
        return copyArray(x);
    } else if (typeof x === "object") {
        return copyObj(x);
    } else {
        return x;
    }
}

function copyArray(obj) {
    let temp = [];
    for (let i of obj) {
        temp.push(deepCopy(i));
    }

    return temp;
}

function copyObj(obj) {
    let temp = {};
    for (let key in obj) {
        temp[key] = deepCopy(obj[key]);
    }

    return temp;
}

export default deepCopy;