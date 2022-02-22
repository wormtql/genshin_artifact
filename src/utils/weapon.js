import { weaponData } from "@asset/weapon"

export function getWeaponDefaultConfigWasmInterface(name) {
    const data = weaponData[name]
    const hasConfig = data.configs && data.configs.length > 0

    if (hasConfig) {
        let defaultConfigs = {}
        for (let c of data.configs) {
            defaultConfigs[c.name] = c.default
        }

        return {
            [name]: defaultConfigs
        }
    } else {
        return "NoConfig"
    }
}
