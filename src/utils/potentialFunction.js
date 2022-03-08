import { potentialFunctionData } from "@potentialFunction"

export function getPotentialFunctionDefaultConfig(name) {
    const item = potentialFunctionData[name]
    if (!item) {
        return "NoConfig"
    }

    let configConfigs = item.config ?? []
    if (configConfigs.length === 0) {
        return "NoConfig"
    }

    let defaultConfigs = {}
    for (let config of configConfigs) {
        defaultConfigs[config.name] = config.default
    }

    return {
        [name]: defaultConfigs
    }
}
