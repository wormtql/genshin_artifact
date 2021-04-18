export default function (obj, path) {
    let temp = path.split(".");

    let ans = obj;
    for (let p of temp) {
        ans = ans[p];
    }

    return ans;
}