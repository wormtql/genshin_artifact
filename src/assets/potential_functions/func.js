let cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let item = r(path).default;
        cache[item.name] = item;
    }
}

importAll(require.context(".", true, /\.po\.func\.js$/));

export default cache;