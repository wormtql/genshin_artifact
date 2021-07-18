let r = require.context("./data", true, /\.js$/);

let data = {};

for (let key of r.keys()) {
    let item = r(key).default;
    data[item.name] = item;
}

export default data;