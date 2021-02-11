let cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let temp = r(path).default;
        cache[temp.name] = temp;
    }
}

importAll(require.context("./data", true, /index\.js$/));
export const weaponsData = cache;