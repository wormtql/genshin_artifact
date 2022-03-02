import { targetFunctionData } from "@targetFunction"

export function upgradeTargetFunctionConfig(name, oldConfig) {
    if (!name) {
        return "NoConfig"
    }

    const data = targetFunctionData[name]
    if (!data) {
        return "NoConfig"
    }

    const configs = data.config ?? []
    if (configs.length === 0) {
        return "NoConfig"
    }

    if (Object.prototype.hasOwnProperty.call(oldConfig, name)) {
        oldConfig = oldConfig[name]
    } else {
        oldConfig = {}
    }
    let newConfig = {}
    for (let c of configs) {
        const configName = c.name
        if (Object.prototype.hasOwnProperty.call(oldConfig, configName)) {
            newConfig[configName] = oldConfig[configName]
        } else {
            newConfig[configName] = c.default
        }
    }
    // console.log(oldConfig, newConfig)

    return {
        [name]: newConfig
    }
}
