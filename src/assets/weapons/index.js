let cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let temp = r(path).default;
        cache[temp.name] = temp;
    }
}

importAll(require.context("./data", true, /index\.js$/));


let _weaponsByType = {
    "sword": [],
    "sword2": [],
    "book": [],
    "stick": [],
    "bow": [],
    "any": [],
};
Object.values(cache).forEach(item => {
    _weaponsByType[item.type].push(item);
});
for (let weaponType in _weaponsByType) {
    _weaponsByType[weaponType].sort((a, b) => {
        return b.star - a.star;
    });
}


export const weaponsData = cache;
export const weaponsByType = _weaponsByType;