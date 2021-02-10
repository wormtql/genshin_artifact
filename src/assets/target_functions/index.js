let cache = {};

function importAll(r) {
    r.keys().forEach(path => {
        let temp = r(path).default;
        cache[temp.name] = temp;
    })
}

importAll(require.context("./data", true, /\.js$/));

export const targetFunctionsData = cache;