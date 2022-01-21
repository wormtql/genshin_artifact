function importWeapons(r) {
    let temp = {}
    for (let path of r.keys()) {
        let data = r(path).default
        temp[data.name] = data
    }
    return temp
}

export const weaponData = importWeapons(require.context("./data", true, /index\.js$/))

function importConfigs(r) {
    let temp = {}
    for (const path of r.keys()) {
        const importt = r(path)
        const weaponName = importt.weaponName

        temp[weaponName] = importt.default
    }

    return temp
}

export const weaponConfig = importConfigs(require.context("./data", true, /\.wcfg\.yaml$/))

let _weaponsByType = {
    "Sword": [],
    "Claymore": [],
    "Catalyst": [],
    "Polearm": [],
    "Bow": []
}
for (let weapon of Object.values(weaponData)) {
    const type = weapon.type
    if (Object.prototype.hasOwnProperty.call(_weaponsByType, type)) {
        _weaponsByType[type].push(weapon)
    }
}
for (let weaponType in _weaponsByType) {
    _weaponsByType[weaponType].sort((a, b) => {
        if (b.star !== a.star) {
            return b.star - a.star
        } else {
            return a.chs.localeCompare(b.chs)
        }
    });
}

export const weaponByType = _weaponsByType;