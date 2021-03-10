let cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let item = r(path).default;
        cache[item.name] = item;
    }
}

importAll(require.context(".", true, /\.po\.config\.js$/));

export default cache;