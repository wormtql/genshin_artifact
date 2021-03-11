let cache = {};

function importAll(r) {
    r.keys().forEach(path => {
        let temp = r(path).default;
        cache[temp.name] = temp;
    })
}

importAll(require.context("./data", true, /\.tf\.js$/));

export default cache;