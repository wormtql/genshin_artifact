import _weaponData from "!../../loaders/meta_loader.js?type=weapon!./weapon_meta_data_auto"

export const weaponData = _weaponData

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