function importAll(r) {
    let temp = {}
    for (let path of r.keys()) {
        let im = r(path).default
        temp[im.name] = im
    }
    return temp
}

export const targetFunctionData = importAll(require.context("./data", true, /\.tf\.js$/))

let _tfByCharacterName = {}

for (let targetFunction of Object.values(targetFunctionData)) {
    if (!Object.prototype.hasOwnProperty.call(_tfByCharacterName, targetFunction["for"])) {
        _tfByCharacterName[targetFunction["for"]] = []
    }
    _tfByCharacterName[targetFunction["for"]].push(targetFunction)
}

export const targetFunctionByCharacterName = _tfByCharacterName