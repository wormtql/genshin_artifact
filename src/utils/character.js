import { characterData } from "@asset/character"

export function getCharacterDefaultConfigWasmInterface(name) {
    const data = characterData[name]
    const hasConfig = data.config && data.config.length > 0
    if (hasConfig) {
        let defaultConfigs = {}
        for (let c of data.config) {
            defaultConfigs[c.name] = c.default
        }

        return {
            [name]: defaultConfigs
        }
    } else {
        return "NoConfig"
    }
}
