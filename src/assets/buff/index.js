const cache = {};

function importAll(r) {
    for (let path of r.keys()) {
        let data = r(path).default;
        cache[data.name] = data;
    }
}

importAll(require.context("./data", true, /\.buff\.js$/));

export default cache;