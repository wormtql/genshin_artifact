let cache = {}
function importAll(r) {
    for (let path of r.keys()) {
        let data = r(path).default;
        let name = data.name.split(".")[0].toLowerCase();
        cache[name] = data;
    }
}

importAll(require.context("./data", true, /\.calculator\.vue$/));
// console.log(cache);
export default cache;