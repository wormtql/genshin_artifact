import _tfData from "!../../loaders/tf_meta_loader.js!./meta"
Object.freeze(_tfData)

// export const targetFunctionData = importAll(require.context("./data", true, /\.tf\.js$/))
export const targetFunctionData = _tfData

let _tfByCharacterName = {}

for (let targetFunction of Object.values(targetFunctionData)) {
    if (!Object.prototype.hasOwnProperty.call(_tfByCharacterName, targetFunction["for"])) {
        _tfByCharacterName[targetFunction["for"]] = []
    }
    _tfByCharacterName[targetFunction["for"]].push(targetFunction)
}

Object.freeze(_tfByCharacterName)

export const targetFunctionByCharacterName = _tfByCharacterName