let cache = {};

function importAll(r) {
    r.keys().forEach(path => {
        let temp = r(path).default;
        cache[temp.name] = temp;
    })
}

importAll(require.context("./data", true, /\.tf\.func\.js$/));

export default cache;