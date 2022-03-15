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

export function upgradeCharacterConfig(name, oldConfig) {
    if (!name) {
        return "NoConfig"
    }

    const data = characterData[name]
    if (!data) {
        return "NoConfig"
    }

    const config = data.config ?? []
    if (config.length === 0) {
        return "NoConfig"
    }

    if (Object.hasOwnProperty.call(oldConfig, name)) {
        oldConfig = oldConfig[name]
    } else {
        oldConfig = {}
    }
    let newConfig = {}
    for (let c of config) {
        const configName = c.name
        if (Object.prototype.hasOwnProperty.call(oldConfig, configName)) {
            newConfig[configName] = oldConfig[configName]
        } else {
            newConfig[configName] = c.default
        }
    }

    return {
        [name]: newConfig
    }
}
