let cache = {}
function importAll(r) {
    for (let path of r.keys()) {
        let data = r(path).default;
        cache[data.name] = data;
    }
}

importAll(require.context("./data", true, /index\.js$/));

let _charactersByElement = {
    fire: [],
    ice: [],
    wind: [],
    // grass: [],
    thunder: [],
    water: [],
    rock: [],
}
Object.values(cache).forEach(item => {
    _charactersByElement[item.element].push(item);
});
for (let ele in _charactersByElement) {
    _charactersByElement[ele].sort((a, b) => {
        return b.star - a.star;
    })
}


export const charactersData = cache;
export const charactersByElement = _charactersByElement;